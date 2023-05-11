use anyhow::{Context, Result};
use colored::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: Option<String>,
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Conversation {
    stream: bool,
    model: String,
    messages: Vec<Message>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let open_ai_key = std::env::var("OPEN_AI_KEY")?;

    let mut conversation = Conversation {
        stream: true,
        model: "gpt-3.5-turbo".to_string(),
        messages: Vec::new(),
    };

    println!(
        "\n{}\n{}\n",
        "ChatGPT in your CLI".blue(),
        "Type 'exit' to quit".blue()
    );

    let stdin = std::io::stdin();
    let mut run = true;
    let mut line = String::new();

    while run {
        talk(&format!("[{}]: ", "Me".green()));

        stdin.read_line(&mut line)?;
        let trimmed = line.trim();

        if trimmed == "exit" {
            run = false;
            talk(&format!("{}\n", "Goodbye!".blue()));
        } else {
            ask_chat_gpt(&open_ai_key, &mut conversation, &trimmed).await?;
        }

        line.clear();
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    delta: Message,
    index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionResponse {
    id: String,
    choices: Vec<Choice>,
}

async fn ask_chat_gpt(key: &str, chat: &mut Conversation, question: &str) -> Result<()> {
    chat.messages.push(Message {
        role: Some("user".to_string()),
        content: Some(question.to_string()),
    });

    let client = reqwest::Client::new();

    talk(&format!("[{}]: ", "ChatGPT".yellow()));

    let mut message = Message {
        role: Some("assistant".to_string()),
        content: Some(String::new()),
    };

    let mut resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&chat)
        .bearer_auth(key)
        .send()
        .await?
        .bytes_stream();

    while let Some(item) = resp.next().await {
        let bytes = item?;
        let data = std::str::from_utf8(&bytes)?.to_string();

        for mut line in data.lines() {
            line = line.strip_prefix("data: ").unwrap_or(line);
            line = line.trim();

            if line == "[DONE]" {
                talk("\n");
                chat.messages.push(message);
                return Ok(());
            }

            if line.is_empty() {
                continue;
            }

            let resp: CompletionResponse = serde_json::from_str(&line)?;

            if let Some(msg) = &resp.choices.get(0).context("No choices")?.delta.content {
                if let Some(content) = message.content.as_mut() {
                    content.push_str(msg);
                }
                talk(msg);
            }
        }
    }

    Ok(())
}

fn talk(text: &str) {
    print!("{}", text);
    stdout().flush().expect("Could not flush stdout");
}

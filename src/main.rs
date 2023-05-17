use anyhow::{Context, Result};
use colored::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::io::{stdout, BufRead, Write};

#[derive(Serialize, Deserialize)]
struct Config {
    version: u8,
    open_ai_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            open_ai_key: String::new(),
        }
    }
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let piped = !atty::is(atty::Stream::Stdin);

    let args: Vec<String> = std::env::args().collect();
    let help = args.iter().find(|arg| {
        let arg = arg.to_ascii_lowercase();
        arg == "--help" || arg == "-h"
    });

    if help.is_some() {
        print_help();
        return Ok(());
    }

    let clear = args.iter().find(|arg| {
        let arg = arg.to_ascii_lowercase();
        arg == "--clear" || arg == "-c"
    });

    let mut cfg: Config = confy::load("chatgpt-cli", None)?;

    if clear.is_some() {
        cfg.open_ai_key = String::new();
        confy::store("chatgpt-cli", None, cfg)?;
        println!("Config cleared.");
        return Ok(());
    }

    let mut initial: Option<&str> = None;

    if args.len() == 2 {
        initial = Some(&args[1]);
    }

    if cfg.open_ai_key.is_empty() {
        prompt_for_api_key(&mut cfg)?;
    }

    let open_ai_key = cfg.open_ai_key;

    let mut conversation = Conversation {
        stream: true,
        model: "gpt-3.5-turbo".to_string(),
        messages: Vec::new(),
    };

    println!("\n{}", "ChatGPT CLI".blue(),);

    if !piped {
        println!("{}", "Type 'exit' to quit".blue());
    }

    println!("");

    let stdin = std::io::stdin();
    let mut run = true;
    let mut line = String::new();

    while run {
        let mut handle = stdin.lock();
        talk(&format!("[{}]: ", "Me".green()));

        if let Some(first) = initial {
            let first = first.trim();
            talk(&format!("{}\n", first));
            ask_chat_gpt(&open_ai_key, &mut conversation, first).await?;
            initial = None;
            continue;
        }

        handle.read_line(&mut line)?;
        let trimmed = line.trim();

        match trimmed {
            "exit" => {
                run = false;
                talk(&format!("{}\n", "Goodbye!".blue()));
                continue;
            }
            "" => continue,
            _ => {
                if piped {
                    talk(&format!("{}\n", trimmed));
                }

                ask_chat_gpt(&open_ai_key, &mut conversation, &trimmed).await?;

                if piped {
                    return Ok(());
                }
            }
        }

        line.clear();
    }

    Ok(())
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

fn prompt_for_api_key(cfg: &mut Config) -> Result<()> {
    println!(
        "{}",
        "Looks like you haven't configured your API key yet.".yellow()
    );
    println!(
        "Go to {} to get your key.",
        "https://platform.openai.com/account/api-keys".bold()
    );
    println!("The key is stored on your machine and only used to talk to the OpenAI API.\n");

    talk("Enter your API key: ");

    let mut key = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut key)?;
    let key = key.trim();

    if key.len() <= 3 || !key.starts_with("sk-") {
        anyhow::bail!("Invalid API key");
    }

    cfg.open_ai_key = key.to_string();

    confy::store("chatgpt-cli", None, cfg)?;

    Ok(())
}

fn print_help() {
    let i = "\x20\x20";
    print!(
        "ChatGPT CLI\n\n\
         Usage: chatgpt [PROMPT] [OPTIONS]\n\n\
         Options:\n\
         {i}-h, --help\t\tPrints help information\n\
         {i}-c, --clear\t\tClears the API key from the config\n\
         \n\
         Examples:\n\
         {i}chatgpt \"How do I write quick sort in Typescript?\"\n\
         {i}chatgpt --clear\n\
         {i}chatgpt --help\n\
         {i}chatgpt\n\
         {i}chatgpt < prompt.txt\n\
         {i}echo \"hi!\" | chatgpt\n"
    );
}

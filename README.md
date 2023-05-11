# chatgpt-cli

Talk to ChatGPT in your terminal.

![Demo](./demo.gif)

## Why

I made this because most of the CLI apps that intergrated with [OpenAI's chat completion](https://platform.openai.com/docs/guides/chat) did not maintain context in conversation. Each prompt started a new one.

This one will keep the context of the conversation just like https://chat.openai.com does.

## Getting Started

1. Clone repo
2. Copy `.env.sample` to `.env` and add your own [OpenAI API key](https://platform.openai.com/account/api-keys).
3. (Alternativly supply key via `OPEN_AI_KEY=` environment variable.)
4. `cargo run`

## Limitations

If you want to `cargo build --release` and copy the bin file to your path so you can use it globally, you'll need to manually add `OPEN_AI_KEY` into your environment variables. It will not read the `.env` in that case.

I plan to fix this by creating a system wide config file that can store your key. It requires turning this into more of a CLI app where it will prompt you to enter your key, etc. etc.

# chatgpt-cli

<p align="center">
  Talk to ChatGPT in your terminal.
</p>

<p align="center">
  <img src="./demo.gif">
</p>

## Why

I made this because most of the CLI apps that intergrated with [OpenAI's chat completion](https://platform.openai.com/docs/guides/chat) did not maintain context in conversation. Each prompt started a new one.

This one will keep the context of the conversation just like https://chat.openai.com does.

## Features

- Streams responses like ChatGPT
- Maintains context in conversation
- Supports piping and redirection

## Getting Started

These instructions are for Mac OS only.

1. Run `sh ./build-and-install.sh`
1. Make sure `/usr/local/bin` is inside your `$PATH`
1. Run `chatgpt` to get started

## Uninstalling

1. `chatgpt --clear`
1. `rm /usr/local/bin/chatgpt`

## Help

```bash
> chatgpt --help
ChatGPT CLI

Usage: chatgpt [PROMPT] [OPTIONS]

Options:
  -h, --help            Prints help information
  -c, --clear           Clears the API key from the config

Examples:
  chatgpt --help
  chatgpt --clear
  chatgpt "How do I write quick sort in Typescript?"
  chatgpt < prompt.txt
  echo "Hi!" | chatgpt
```

## License

MIT
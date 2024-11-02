# Rust Discord Bot
A Discord bot made in Rust with IA functionalities.

## How Install
Simple way to install and run this project on your computer.
### Install LLAMA3 by docker
I Recomend to use a computer with GPU, if you don't have it, remove the `--gpus=all` from the first command
```bash
  docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
  docker exec -it ollama ollama run llama3
```
More informatation on: https://hub.docker.com/r/ollama/ollama
### Project clone
```bash
  git clone https://github.com/eidiinnn/rust-discord-bot
  cd rust-discord-bot
```

### .env file Setup
```
GUILD_ID=The Discord server guild
TOKEN=Your Discord token
LLAMA_API_URL=http://localhost:11434/api/generate
LLAMA_MODEL=llama3
```

### Start the project
```bash
  cargo run
```
### Bot usage
Go to any chat on GUILD_ID Discord server you put, and use / in the chat to see the available commands.
    

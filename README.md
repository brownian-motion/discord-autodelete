# Event Horizon

This is a Discord bot that slowly deletes all posts in a channel as they expire, sliding inexorably into oblivion.
You can use it to remove all posts that live longer than a certain age.
This is useful for things like `#venting` channels, selfies, and anything sensitive you don't want years of history getting exposed in the future.

## Goals
- [ ] Configuration
  - [x] Load the "delete config" from a human-readable file on startup
  - [ ] Write the "delete config" to a file if updated using `/commands`
  - [x] Load discord bot login tokens from a configured file
  - [ ] Write the "delete config" to a file if updated using API endpoints by an authenticated user
  - [ ] Back up the config file periodically, if changed
- [x] Deleting
  - [x] Be able to fetch a list of posts in a channel older than the configured timeout
  - [x] Be able to delete all posts in a channel older than the configured timeout
  - [x] "Dry run" (read-only) mode to list what WOULD be deleted without deleting anything
  - [x] Don't delete pinned messages
  - [x] Be able to only delete messages with images/video, leaving text-only messages intact
- [ ] Running
  - [ ] Add a `/command` to trigger this manually from discord
  - [ ] Add a `/command` to change the configuration manually from discord
  - [x] Have it poll periodically in a loop
  - [x] Use `docker` and `cron` to schedule this to run periodically
  - [x] Be able to edit the config externally without having to restart the container
  - [ ] Restructure into a "scheduler" and "worker" architecture.
      - Instead of doing all of the work in order on one global timer, have 1 scheduler process that schedules work for other workers on other threads to carry out.
      - This would require a not-insignificant rewrite
      - This would require computing a diff between what's scheduled and what the config says to schedule for
- [ ] Nice-to-haves
  - [x] Anyone can technically host this
  - [x] Config files are bootstrapped into existence
  - [x] Document how to host this
  - [ ] Write a short script to make it easier to create a new self-hosted instance of this bot
  - [ ] Tests!

## License
Licensed under GPL-3. Please feel free to copy and modify this, and host it yourself.

## Contributing
Please open an issue or a pull request!

## Installation and Deployment
Anyone can download and run this bot themselves!
Just make sure you've set up a discord bot first, grabbed its token, and added it with sufficient permissions to your Discord server. You need permissions to "read message history" and "manage messages".

// TODO: add an image showing what permissions to set!

### Using Cargo and Rust:
```sh
cargo run -- --config-path="./config.yaml" --discord-bot-token-path="./token.txt"
```

For details, run `cargo run -- --help`.

### Using Nix:
```sh
nix run -- --config-path="./config.yaml" --discord-bot-token-path="./token.txt"
```

### Using docker-compose:
```yaml
secrets:
  discord_bot_token:
    file: ./token.txt

services:
  discord_autodelete:
    container_name: "discord-autodelete"
    build: https://github.com/brownian-motion/discord-autodelete.git
    restart: unless-stopped
    network_mode: host
    volumes:
      - "./config:/app/config"
    environment:
      - "CONFIG_PATH=/app/config/config.yaml"
      - "DISCORD_BOT_TOKEN_PATH=/run/secrets/discord_bot_token"
    secrets:
      - discord_bot_token # mounted into /run/secrets/discord_bot_token
```

### Configuration
This bot relies on two configuration files: a file with the bot token allowing access to the API, and a config file containing per-channel delete schedules.

#### Discord Bot Token
Just paste your discord bot token, on a single line, into a single file, and direct the program to it using the `DISCORD_BOT_TOKEN_PATH` environment variable or the `--discord-bot-token-path` flag.

#### Config
Your config file will be created for you if it does not already exist, and you can edit it while the app is running (changes will be picked up and applied on the next run).

Config files should have the following format:

```yaml
guilds:
- id: '2417843429083125945'
  channels:
  # delete everything from channel <#1641798796715016192> that gets older than 2 minutes:
  - id: '1641798796715016192'
    delete_older_than:
      minutes: 2
  # delete every message with images/videos from channel <#637575874339525219> that gets older than a day and a half:
  - id: '637575874339525219'
    just_images: true
    delete_older_than:
      days: 1
      hours: 12
```

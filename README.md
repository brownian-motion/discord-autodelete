# Discord Auto-Delete

This is the implementation of a bot that deletes posts from configured channels.

## Goals
- [ ] Configuration
  - [x] Load delete config from a human-readable file on startup
  - [ ] Write delete config to a file if updated using `/commands`
  - [x] Load discord bot login tokens from a configured file
- [ ] Deleting
  - [x] Be able to fetch a list of posts in a channel older than the configured timeout
  - [x] Be able to delete all posts in a channel older than the configured timeout
  - [x] "Dry run" mode to list what WOULD be deleted without deleting anything
  - [x] Don't delete pinned messages
- [ ] Running
  - [ ] Add a `/command` to trigger this manually from discord
  - [ ] Add a `/command` to change the configuration manually from discord
  - [x] Have it poll periodically in a loop
  - [ ] Use `docker` and `cron` to schedule this to run periodically
- [ ] Nice-to-haves
  - [ ] Write a short script to make it easier to create a new self-hosted instance of this bot
  - [ ] Tests!


## License
Licensed under GPL-3. Please feel free to copy and modify this, and host it yourself.

## Contributing
Please open an issue or a pull request!

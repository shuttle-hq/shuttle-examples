# Serenity Hello World Bot with Shuttle

## Introduction

In this example we will deploy a Serenity bot with Shuttle that responds to the `!hello` command with `world!`. To run this bot we need a valid Discord Token. To get started log in to the [Discord developer portal](https://discord.com/developers/applications).

## Features

- A Discord bot that responds to `!hello` with `world!`.

## Pre-requisites

- Rust
- Discord account
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

1. Click the New Application button, name your application and click Create.
2. Navigate to the Bot tab in the lefthand menu, and add a new bot.
3. On the bot page click the Reset Token button to reveal your token. Put this token in your `Secrets.toml`. It's very important that you don't reveal your token to anyone, as it can be abused. Create a `.gitignore` file to omit your `Secrets.toml` from version control.
4. For the sake of this example, you also need to scroll down on the bot page to the Message Content Intent section and enable that option.

To add the bot to a server we need to create an invite link.

1. On your bot's application page, open the OAuth2 page via the lefthand panel.
2. Go to the URL Generator via the lefthand panel, and select the `bot` scope as well as the `Send Messages` permission in the Bot Permissions section.
3. Copy the URL, open it in your browser and select a Discord server you wish to invite the bot to.

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo](https://github.com/serenity-rs/serenity) for more examples.

Run the project with `shuttle run`.

Writing `!hello` into a channel your bot has access to will make it respond with `world!`.

When you're ready to deploy, use `shuttle deploy`.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're having trouble getting your bot to work, try checking your Discord token or your bot's intents via the Discord dev portal.

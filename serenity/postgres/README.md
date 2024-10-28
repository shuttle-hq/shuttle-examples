# Serenity Todo List Bot with Shuttle

In this example we will deploy a Serenity bot with Shuttle that can add, list and complete todos using [Application Commands](https://discord.com/developers/docs/interactions/application-commands). To persist the todos we need a database. We will have Shuttle provison a PostgreSQL database for us by enabling the `sqlx-postgres` feature for `shuttle-service` and passing `#[shared::Postgres] pool: PgPool` as an argument to our `main` function.

## Features

- A Discord bot that adds application commands for adding, completing and listing to-dos.

## Pre-requisites

- Rust
- Discord account
- Docker if running locally
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

For this example we also need a `GuildId`.

1. Open your Discord client, open the User Settings and navigate to Advanced. Enable Developer Mode.
2. Right click the Discord server you'd like to use the bot in and click Copy Id. This is your Guild ID.
3. Store it in `Secrets.toml` and retrieve it like we did for the Discord Token.

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo](https://github.com/serenity-rs/serenity) for more examples.

Run the project with `shuttle run`.

Go back to your Discord server where your bot was invited to and try out some of the application commands.

When you're ready to deploy, use `shuttle deploy`.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're running locally don't ofrget to have Docker running!
- If you're having trouble getting your bot to work, try checking your Discord token or your bot's intents via the Discord dev portal.

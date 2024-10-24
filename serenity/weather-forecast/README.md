# Serenity Weather Forecast Bot with Shuttle

A Discord bot created with Serenity that can get the weather forecast.

## Features

- A Discord bot that can report the weather forecast.

## Pre-requisites

- Rust
- Discord account
- [accuweather.com](accuweather.com) API key
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

For a full tutorial on how to build and set up this bot, please refer to [Shuttle docs](https://docs.shuttle.dev/templates/tutorials/discord-weather-forecast)

When you're ready to deploy, use `shuttle deploy`.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're having trouble connecting to Discord, don't forget to check your `Secrets.toml` file and your bot's intents.
- If you're having trouble connecting to Accuweather, don't forget to check your `Secrets.toml` file.

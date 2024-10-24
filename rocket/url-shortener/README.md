# Rocket URL Shortener

## Introduction

A URL shortener that you can use from your terminal - built with Shuttle, rocket and postgres/sqlx.

## Features

- Create shortened URLs from links!

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run this project with `shuttle run`.

Once running, you can use this URL shortener directly from your terminal. Just copy and paste this command to your terminal and replace `<URL>` with the URL that you want to shorten

```bash
curl -X POST -d '<URL>' https://s.shuttleapp.rs
```

like this

```bash
curl -X POST -d 'https://docs.rs/shuttle-service/latest/shuttle_service/' https://s.shuttleapp.rs
```

you will get the shortened URL back (something like this `https://s.shuttleapp.rs/RvpVU_`)

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If running locally, don't forget to make sure Docker is installed and running!

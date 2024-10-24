# Axum with Google OAuth

## Introduction

This template is an example of how you can implement Google OAuth using the Axum web framework in Rust.

## Features

- Google-based Oauth2 authentication

## Pre-requisites

- Rust
- Google account
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Make sure you set up your Google OAuth, which you can find a link to set up [here.](https://console.cloud.google.com/apis/dashboard)

Set your secrets in a `Secrets.toml` file:

```toml
GOOGLE_OAUTH_CLIENT_ID = "your-client-id"
GOOGLE_OAUTH_CLIENT_SECRET = "your-client-secret"
```

Run the project with `shuttle run`.

Visit `http://localhost:8000` once the app is running, then try it out!

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.

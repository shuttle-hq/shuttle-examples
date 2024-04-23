# Google OAuth in Axum

This template is an example of how you can implement Google OAuth using the Axum web framework in Rust.

## Prerequisites

Make sure you set up your Google OAuth, which you can find a link to set up [here.](https://console.cloud.google.com/apis/dashboard)

Set your secrets in the Secrets.toml file:

```toml
GOOGLE_OAUTH_CLIENT_ID = "your-client-id"
GOOGLE_OAUTH_CLIENT_SECRET = "your-client-secret"
```

Use `cargo shuttle run` and visit `http://localhost:8000` once the app is running, then try it out!

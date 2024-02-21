## OAuth Axum Rust Example
This repo is an example of how you can quickly and easily implement OAuth using the Axum web framework in Rust. Hosted on Shuttle.

### How to Run
Make sure you set up your Google OAuth, which you can find a link to set up [here.](https://console.cloud.google.com/apis/dashboard)

Git clone the repo:
```sh
git clone https://github.com/joshua-mo-143/oauth-axum-shuttle-ex.git
cd oauth-axum-shuttle-ex
```

Set your secrets in the Secrets.toml file:
```toml
GOOGLE_OAUTH_CLIENT_ID = "your-client-id"
GOOGLE_OAUTH_CLIENT_SECRET = "your-client-secret"
```

Use `cargo shuttle run` and visit `http://localhost:8000` once the app is running, then try it out!


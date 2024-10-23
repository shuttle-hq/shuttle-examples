# Axum with OpenAI

A simple endpoint that sends a chat message to ChatGPT and returns the response.

## How to use this template

Set your OpenAI API key in `Secrets.toml`, then try it on a local run with:

```sh
curl http://localhost:8000 -H 'content-type: application/json' --data '{"message":"What is shuttle.rs?"}'
```

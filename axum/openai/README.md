A simple endpoint that sends a chat message to ChatGPT and returns the response.

Set your OpenAI API key in `Secrets.toml`, then try it on a local run with:

```sh
curl http://localhost:8000 -H 'content-type: application/json' --data '{"message":"What is shuttle.rs?"}'
```

## Shuttle AI Playground
This template enables you to spin up an AI playground in the style of OpenAI.

## Features
- Frontend (via Next.js)
- Authentication and sign-ups with cookie-wrapped JWTs
- Database
- OpenAI

## How to use
Before using this, you will need an OpenAI API key.

1) Ensure the OpenAI API key is in your `Secrets.toml` file (see file for syntax if not sure how to use).
2) Run `npm --prefix frontend install && npm --prefix frontend run build` to build the Next.js frontend.
3) Use `shuttle run` to run the template locally - or use `shuttle deploy` to deploy!

## Troubleshooting
- The default port is at 8000. If you are already running something here, you can use `--port` to select a different port.
- Your OpenAI client may error out if you don't have your OpenAI API key set correctly (should be `OPENAI_API_KEY` in Secrets.toml).

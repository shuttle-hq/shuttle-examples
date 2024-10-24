# Actix Web with Clerk auth

## Introduction

This template uses Actix-web and the clerk-rs crate for providing authentication from Clerk in the backend.

The frontend is a simple React app that uses Clerk's React components.

The template simply shows a list of all signed up users after you sign in, but it can be extended to create a full app with authenticated endpoints.

## Features

- Authentication backed by Clerk
- React-based frontend (with Clerk React components)

## Pre-requisites

- Rust
- Node.js/NPM.
- Typescript.
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Sign up at [https://clerk.com](https://clerk.com), then go to the dashboard and create an application with the sign-in options you prefer.

Get the `CLERK_SECRET_KEY` secret and put it in `backend/Secrets.toml`.
Make sure you don't commit this file.

Get the `VITE_CLERK_PUBLISHABLE_KEY` secret and put it in `frontend/.env`.

> The frontend was initialized from the React+TS template in `npm create vite@latest`.
> The Clerk components were then added by following the guide at <https://clerk.com/docs/quickstarts/react>.

`cd` into the frontend and run `npm install` and `npm run build`. This builds the frontend assets and places them in `frontend/dist`.

cd back to the root and run `shuttle run`.
The backend serves the web page from the dist folder, and an auth-protected API that fetches the list of all users that have signed up so far.

Once you're ready to deploy, use `shuttle deploy`. You may want to consider switching to a production key from Clerk if you aren't using one already.
The development key can still be used in `Secrets.dev.toml`.

## Troubleshooting
- If connecting to Clerk doesn't work, try checking your Secrets.toml file.
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.

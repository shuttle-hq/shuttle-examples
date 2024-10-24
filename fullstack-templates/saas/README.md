# Shuttle SaaS Example

## Introduction

This template is a SaaS template with a Next.js Typescript frontend and a Rust backend.
The design of the template internally is based on a sales-oriented Customer Relationship Management (CRM) tool where
users will be able to view their customers, sales records as well as some analytics.

## Features

- Take subscription payments with Stripe
- Email session-based login
- Mailgun (email subscription, welcome email etc)
- Pre-configured frontend routes for easy transition
- Examples of how to implement simple dashboard analytics
- Uses `turbowatch` for live reloads (with `npm run dev`)
- Bundle analysis with `npm run analyze`

![Main page for Next.js + Shuttle Saas Template](./Mainpage.png)

## Pre-requisites

- Rust
- Node.js/NPM
- Typescript
- Docker if running locally
- [Mailgun API key](https://www.mailgun.com)
- [Stripe API key](https://stripe.com)
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run `npm i` to install the dependencies on the frontend.

Set your secrets in the Secrets.toml file at the `Cargo.toml` level of your backend folder. Unset secrets will default
  to "None" to prevent automatic crashing of the web service, but some services may not work.

Use `npm run dev` and visit <http://localhost:8000> to try it out.

When you're ready to deploy, use `npm run deploy` which will build the static assets then deploy the service.

## Troubleshooting

- If you change the migrations after running locally or deploying, you will need to go into the database itself and
  delete the tables. You can do this easily with something
  like [psql](https://www.postgresql.org/docs/current/app-psql.html) or [pgAdmin](https://www.pgadmin.org/).
- If connecting to external services like Stripe doesn't work, try checking your Secrets.toml file.
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.

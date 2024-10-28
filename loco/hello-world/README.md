# Shuttle Loco template

## Introduction

Welcome to the [Loco](https://loco.rs) Hello World Shuttle Template, designed with a focus on controllers and views (response schema). This minimalistic template comes with several features to kickstart your project:

## Features

* REST API service without a database.
* **Controllers:** Handle web requests parameters, body, validation, and render a response that is content-aware. We use Axum for the best performance, simplicity, and extensibility. Controllers also allow you to easily build middlewares, which can be used to add logic such as authentication, logging, or error handling before passing requests to the main controller actions.
* **Views:** Loco can integrate with templating engines to generate dynamic HTML content from templates.
* **Background Jobs:** Perform compute or I/O intensive jobs in the background with a Redis backed queue, or with threads. Implementing a worker is as simple as implementing a perform function for the Worker trait.
* **Scheduler:** Simplifies the traditional, often cumbersome crontab system, making it easier and more elegant to schedule tasks or shell scripts.
* **Mailers:** A mailer will deliver emails in the background using the existing loco background worker infrastructure. It will all be seamless for you.
* **Storage:** In Loco Storage, we facilitate working with files through multiple operations. Storage can be in-memory, on disk, or use cloud services such as AWS S3, GCP, and Azure.
* **Cache:** Loco provides an cache layer to improve application performance by storing frequently accessed data.

![Main page for Next.js + Shuttle Saas Template](./Mainpage.png)

## Pre-requisites

- Rust
- `loco` cli (for further development)
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000> to try it out.

When you're ready to deploy, use `npm run deploy` which will build the static assets then deploy the service.

## Troubleshooting

- If you change the migrations after running locally or deploying, you will need to go into the database itself and
  delete the tables. You can do this easily with something
  like [psql](https://www.postgresql.org/docs/current/app-psql.html) or [pgAdmin](https://www.pgadmin.org/).
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.

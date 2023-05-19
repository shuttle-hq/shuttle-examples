## Shuttle SaaS Example
### Introduction
This repo is meant to serve as a SaaS template with a Next.js Typescript frontend and a Rust backend. The design of the template internally is based on a sales-oriented Customer Relationship Management (CRM) tool where users will be able to view their customers, sales records as well as some analytics.

### Features
- Take subscription payments with Stripe
- Email session-based login
- Mailgun (email subscription, welcome email etc)
- Pre-configured frontend routes for easy transition
- Examples of how to implement simple dashboard analytics

### Pre-requisites

* Rust

* Node.js/NPM.

* Typescript.

* [cargo-shuttle](https://www.shuttle.rs)

### Instructions for Usage

* Fork or clone the repo, then navigate to the folder where you cloned the repo:

```
  git clone https://github.com/joshua-mo-143/shuttle-saas-example.git
  cd shuttle-saas-example
```

* Run `npm i` to install the dependencies on the frontend. 

* Set your secrets in the Secrets.toml file at the `Cargo.toml` level of your backend folder (any that are unset will default to "None" to stop the web service from automatically crashing but some services may not work!)

* Run `npm run dev` and go to [http://localhost:8000](http://localhost:8000) once the app has built and you should see the following:

![Main page for Next.js + Shuttle Saas Template](./Mainpage.png)

### Troubleshooting

* If you change the migrations after running locally or deploying, you will need to go into the database itself and delete the tables. You can do this easily with something like [psql](https://www.postgresql.org/docs/current/app-psql.html) or [pgAdmin](https://www.pgadmin.org/).

* If connecting to external services like Stripe doesn't work, try checking your Secrets.toml file.

* Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add the `--port <port-number>` to the `cargo shuttle run` command to change this.
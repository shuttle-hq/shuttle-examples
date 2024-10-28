# `shuttle-cron`

## Introduction

A service that prints out a log message at specified cron-style intervals.

This example uses the `apalis` framework to be able to carry out cronjobs. A struct is defined that has some given data in it, and is stored in the Apalis monitor. We also implement a struct that holds a `chrono::DateTime<chrono::Utc>`, which `apalis` uses internally for cron job streaming.

When the cron job is called, the data is grabbed from the `JobContext` and we then execute the job.

The actual function to be ran itself is stored in `job_fn()` in the main function, as a function pointer.

## Features

- A web service that can carry out cron-style scheduled tasks

## Pre-requisites

- Rust
- Docker if running locally
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run `shuttle run` to spin up the service locally.

You can change the behavior of the cronjob by editing the `execute()` function for `CronjobData`.

Note that the run method doesn't have to be an implementation method for `CronjobData` - you can write your own!

When you're ready to deploy, use `shuttle deploy`.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're running locally, don't forget to have Docker installed and running!

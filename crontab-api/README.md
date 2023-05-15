# crontab-api

A service that calls URLs at specified cron intervals.

The service exposes a `/set-schedule` endpoint that accepts a schedule and a URL
as form data and persists the cron job with `shuttle_persist` between runs.

Internally, `CrontabService` implements a custom service with
[`shuttle_runtime::Service`](https://docs.shuttle.rs/examples/custom-service),
usage of [`shuttle_persist`](https://docs.shuttle.rs/resources/shuttle-persist),
and sets up an `axum::Server` that sends jobs to a `CronRunner`.

# Usage
Run `cargo shuttle run` to spin up the service locally.

Fire a request to the `set-schedule` URL specifying a new cron job. Use `request.sh` 
for a quick example or use the below snippet:

```
curl -v http://localhost:8000/set-schedule\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule='*/2 * * * * *'&url='example.com'"
```

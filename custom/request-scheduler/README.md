# request-scheduler

A service that calls URLs at specified cron-style intervals.

The service exposes a `/set-schedule` endpoint that accepts a schedule and a URL
as form data and persists a job tuple `(schedule, url)` with `shuttle_persist` 
between runs, e.g. will pick up existing jobs after being restarted.

Internally, `CrontabService` implements a custom service with
[`shuttle_runtime::Service`](https://docs.shuttle.rs/examples/custom-service),
uses [`shuttle_persist`](https://docs.shuttle.rs/resources/shuttle-persist),
and sets up an [`axum::Server`](https://github.com/tokio-rs/axum) that sends 
jobs to a `CronRunner`.

# Usage
Run `cargo shuttle run` to spin up the service locally.

Fire a POST request to the `set-schedule` URL to create a new cron job. Use 
`request.sh` for a quick example or use the below snippet:

```
curl -v http://localhost:8000/crontab/set\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule='*/2 * * * * *'&url='example.com'"
```

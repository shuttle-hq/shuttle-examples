# request-scheduler

A service that calls URLs at specified cron-style intervals.

The service exposes a `/crontab/set` endpoint that accepts a schedule and a URL
as form data and persists jobs with `shuttle_persist` between runs, e.g. it will 
pick up existing jobs after being restarted.

Internally, `CrontabService` implements a custom service with
[`shuttle_runtime::Service`](https://docs.shuttle.rs/examples/custom-service),
uses [`shuttle_persist`](https://docs.shuttle.rs/resources/shuttle-persist),
and sets up an [`axum::Server`](https://github.com/tokio-rs/axum) that sends 
jobs to a `CronRunner`.

# Usage
Run `cargo shuttle run` to spin up the service locally.

Fire a POST request to the `set-schedule` URL to create a new cron job. Use 
the provided `request.sh` for a quick example or the below snippet:

```
curl -v http://localhost:8000/crontab/set\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule='*/2 * * * * *'&url='http://localhost:8000/trigger-me'"
```

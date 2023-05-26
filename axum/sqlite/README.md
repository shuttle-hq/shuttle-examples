# Using sqlite with Shuttle
This example demonstrates usage of the `shuttle-sqlite` resource.

`shuttle-sqlite` uses [`sqlx`](https://github.com/launchbadge/sqlx) to provide an in-process sqlite database. See 
[the docs](https://docs.rs/shuttle-sqlite/latest/shuttle_sqlite/) for all configuration options.

## Usage

Use `RUST_LOG` to quiet down the incredibly noisy `sqlx`:
```
RUST_LOG="info,sqlx=off" cargo shuttle run
```
Watch the console of navigate to `localhost:8000` to see the output in the browser.
# Persist Example

An example app to show what you can do with shuttle.

## How to deploy the example

```bash
cargo shuttle project start --name=$PROJECT_NAME
cargo shuttle deploy --name=$PROJECT_NAME
```

Once deployed you can post to the endpoint the following values:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"date":"2020-12-22", "temp_high":5, "temp_low":5, "precipitation": 5}' {$PROJECT_NAME}.shuttleapp.rs
```

The json data will then persist within Shuttle it can be queried with the following curl request

```bash
curl {$PROJECT_NAME}.shuttleapp.rs/2020-12-22
```

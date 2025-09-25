# Polars ETL with OpenTelemetry

An ETL pipeline using Polars for data processing and OpenTelemetry for observability, deployable on Shuttle.

## What it does

- Processes CSV data with Polars (load → clean → aggregate → filter/sort → save)
- Exposes HTTP endpoints via Axum
- Emits metrics and traces via OpenTelemetry
- Integrates with Better Stack for monitoring (when deployed on Shuttle)

## Running locally

**ETL benchmark:**
```bash
RUST_LOG=info cargo run --release --features bench-cli
```

**Web service:**
```bash
cargo run --release
```

Place your CSV file at `./data/yellow_tripdata_2015-01.csv` or set `DATA_PATH=/path/to/file.csv`.

## Deploy to Shuttle

```bash
shuttle deploy
```

## Endpoints

- `GET /` - Health check
- `GET /health` - Health check with metrics
- `GET /benchmark` - Returns ETL metrics and emits telemetry events

## Observability

The service emits OpenTelemetry metrics:
- `monotonic_counter.http_requests_total` - Request counts
- `histogram.request_duration_ms` - Request latency
- `histogram.*_time_ms` - ETL stage timings
- `monotonic_counter.rows_processed` - Data throughput

When deployed on Shuttle with Better Stack integration enabled, these automatically appear in your telemetry dashboard.

## Configuration

**Environment variables:**
- `DATA_PATH` - Path to CSV file (default: `data/yellow_tripdata_2015-01.csv`)

**Features:**
- `bench-cli` - Enables local ETL benchmark
- `otel-otlp` - Enables OTLP export (for non-Shuttle deployments)

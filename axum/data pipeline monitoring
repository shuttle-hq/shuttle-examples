
# polars-otel-shuttle

High-performance ETL in Rust with Polars, production-ready observability via OpenTelemetry and tracing, and one-command deploy on Shuttle with built-in export to Better Stack.

## Overview

This repository contains:

- A local, benchmark-style ETL pipeline using Polars over a large NYC Taxi CSV.
- An Axum HTTP service (for Shuttle) that exposes lightweight endpoints and emits structured traces and metrics.
- OTEL integration that requires no collector setup when deployed on Shuttle.
- Examples of metric events that flow to Better Stack (counters and histograms), with guidance for dashboards and log-based queries.

## Why this project

Many Python data teams rely on Pandas for ETL. Rust + Polars delivers multi-threaded performance with memory safety and predictable latency. This project shows how to:

- Build an ETL pipeline in Rust/Polars.
- Instrument it with tracing and OpenTelemetry.
- Export telemetry to Better Stack with Shuttle’s native OTEL support (no sidecar collector).
- Visualize performance using log-derived metrics and dashboards.

In addition, we mention Shuttle’s upcoming LogFire integration as a future option for observability.

## Project layout

```
polars-otel-shuttle/
├─ Cargo.toml
├─ Shuttle.toml
├─ src/
│  ├─ etl.rs              # Polars ETL pipeline (load → clean → aggregate → filter/sort → save)
│  ├─ observability.rs    # Tracing/OTEL setup (fmt logs; optional OTLP for non-Shuttle runs)
│  └─ main.rs             # CLI benchmark (feature-gated) + Axum web app for Shuttle
├─ data/                  # Put the CSV here (ignored by Git)
└─ results/               # Output directory (ignored by Git)
```

## Prerequisites

- Rust toolchain (stable)
- A large CSV to process, e.g. `yellow_tripdata_2015-01.csv` (~1.9GB) placed in `./data/`
- For deploys: Shuttle CLI and a Shuttle account
- For dashboards: a Better Stack account (Shuttle Telemetry export requires Pro tier or above)

> **Note**: This repository does not commit the dataset. Keep large files out of Git.

## Running the local ETL benchmark

The CLI benchmark is feature-gated to keep the Shuttle build lightweight.

From the repo root:

```bash
RUST_LOG=info cargo run --release --features bench-cli
```

By default, the program looks for:

```
data/yellow_tripdata_2015-01.csv
```

Override via environment variable:

```bash
DATA_PATH=/absolute/or/relative/path/to/your.csv \
RUST_LOG=info cargo run --release --features bench-cli
```

### What it does

`src/etl.rs` performs:

1. **Load**: Lazy CSV scan (Polars)
2. **Clean**: Basic data preparation
3. **Aggregate**: Representative group-by computations
4. **Sort & filter**: Derive example counts
5. **Save**: Write results to `./results/` (ignored by Git)

The CLI prints a summary and emits structured logs (JSON) with timing fields.

## The Shuttle web service (observability demo)

The Shuttle build exposes a small Axum API that simulates results and emits tracing events for observability. It avoids running the heavy ETL on ephemeral dynos.

### Endpoints

- `GET /` and `GET /health` — Simple health check (also emits a counter)
- `GET /benchmark` — Returns representative metrics and emits counters and histograms for dashboards

Run locally as a normal binary:

```bash
cargo run --release
```

Deploy to Shuttle:

```bash
cargo shuttle deploy
```

Shuttle will return a public URL like:

```
https://<your-project>.shuttle.app
```

## Observability: tracing + OpenTelemetry

This project uses:

- `tracing` for structured events
- `tracing-subscriber` for JSON logs with RFC3339 timestamps
- `tracing-opentelemetry` to bridge spans/events to OTEL (when applicable)

### How it is initialized

`src/observability.rs` sets up JSON logging for both CLI and server. When running on Shuttle, the platform injects an OTEL exporter that ships your events to the configured Telemetry destination (Better Stack). You don’t need to run or configure a collector.

If you want to export to an OTLP endpoint when not on Shuttle, enable the `otel-otlp` feature and set:

- `OTEL_EXPORTER_OTLP_ENDPOINT` (e.g., `http://localhost:4318`)
- `OTEL_EXPORTER_OTLP_HEADERS` (e.g., `Authorization=Bearer ...`)

```bash
cargo run --release --features otel-otlp
```

## Metric events you can chart

Shuttle’s telemetry pipeline interprets specific `tracing::info!` fields as metrics:

- `monotonic_counter.*` → Cumulative counters (use rate for per-second/minute)
- `counter.*` → Values that can go up/down
- `histogram.*` → Distributions (p50, p95, avg, etc.)

The Axum handlers emit a few examples:

```rust
// Count requests to /benchmark
tracing::info!(monotonic_counter.http_requests_total = 1, route="/benchmark", "Benchmark hit");

// Publish stage timings as histograms (ms)
tracing::info!(histogram.load_time_ms = 1.2_f64 * 1000.0, stage="load", "stage timing");
tracing::info!(histogram.clean_time_ms = 0.8_f64 * 1000.0, stage="clean", "stage timing");
tracing::info!(histogram.aggregate_time_ms = 0.4_f64 * 1000.0, stage="aggregate", "stage timing");
tracing::info!(histogram.sort_filter_time_ms = 0.3_f64 * 1000.0, stage="sort_filter", "stage timing");
tracing::info!(histogram.save_time_ms = 0.1_f64 * 1000.0, stage="save", "stage timing");
tracing::info!(histogram.total_time_ms = 2.8_f64 * 1000.0, stage="total", "stage timing");

// Rows processed (cumulative)
tracing::info!(monotonic_counter.rows_processed = 12_748_986_i64, "rows processed (demo)");

// Request duration per call
let t0 = std::time::Instant::now();
// ... work ...
let elapsed_ms = t0.elapsed().as_millis() as f64;
tracing::info!(histogram.request_duration_ms = elapsed_ms, route="/benchmark", "Benchmark responded");

// Health endpoint heartbeat
tracing::info!(monotonic_counter.health_checks = 1, "health checked");
```

> **Tip**: Keep metric names short and stable; add contextual attributes like `route`, `stage`, or `dataset`.

## Exporting to Better Stack with Shuttle

1. In Better Stack, create a Telemetry Source (OpenTelemetry) and copy the Source Token.
2. In the Shuttle web console → your project → Settings, enable Better Stack and paste the Source Token; save.
3. Deploy (or redeploy) your app:

```bash
cargo shuttle deploy
```

4. Hit the endpoints to generate events:

```bash
curl -s https://<your-project>.shuttle.app/benchmark >/dev/null
curl -s https://<your-project>.shuttle.app/benchmark >/dev/null
curl -s https://<your-project>.shuttle.app/health
```

That’s it—no collector, no sidecars. Shuttle forwards OTEL data to Better Stack.

## Viewing data in Better Stack

### 1. Logs stream (sanity check)

- Better Stack → Telemetry → your Source → Logs
- Search for a field like `histogram.total_time_ms` or `monotonic_counter.http_requests_total`
- You should see JSON events with your attributes (`route`, `stage`, etc.)

### 2. Dashboards

Create a dashboard and add charts that query your metrics:

#### Requests per route
- **Metric**: `monotonic_counter.http_requests_total`
- **Aggregation**: Rate (per 1m or 5m)
- **Group by**: `route`
- **Visualization**: Line

#### Request duration
- **Metric**: `histogram.request_duration_ms`
- **Aggregation**: p95 (or avg, p50)
- **Group by**: `route`
- **Visualization**: Line

#### ETL stage timings
- **Metrics**: `histogram.load_time_ms`, `histogram.clean_time_ms`, `histogram.aggregate_time_ms`, `histogram.sort_filter_time_ms`, `histogram.save_time_ms`
- **Aggregation**: Avg
- **Group by**: `stage`
- **Visualization**: Stacked area or multi-line

#### Rows processed
- **Metric**: `monotonic_counter.rows_processed`
- **Aggregation**: Rate to see throughput
- **Visualization**: Bar or line

> **Note**: If your organization uses Metrics (pre-computed time series derived from logs), you can define them for frequently queried fields to reduce query latency and cost. Keep metric cardinality low (limit unique combinations of attributes like `route`, `stage`, `dataset`).

### 3. Explore (ad-hoc)

Use Explore for quick, one-off queries over logs without defining Metrics. Search by your metric field names and filter by attributes.

### 4. Using Better Stack’s built-in telemetry

You should see fields like these in Explore / Logs:

- `duration` (request/operation span duration)
- `service`, `span_name`, `name`
- `http_target` (route), `net_peer_name`
- `error`, `kind`, `tags`

#### Quick dashboards from built-ins

Create a Dashboard → Add chart → Metrics (or start in Explore and “Save as metric”):

1. **HTTP latency (p95)**  
   - **Metric**: `duration`
   - **Aggregation**: p95
   - **Group by**: `http_target` (or `span_name`)
   - **Time grain**: 1m or 5m  
     Use this to see tail latency per route.

2. **Request rate**  
   - **Measure**: `count()` of events
   - **Filter**: `span_name` that corresponds to the HTTP request span (if present), or `http_target` exists
   - **Aggregation**: Rate per minute
   - **Group by**: `http_target`  
     This shows throughput per endpoint.

3. **Error rate**  
   - **Filter**: `error = true` (or `severity_text >= ERROR` if you don’t have `error`)
   - **Measure**: `count()` → rate
   - **Group by**: `http_target`  
     Compare against total requests to get error percentage.

4. **Slowest endpoints (top N)**  
   - **Metric**: `duration`
   - **Aggregation**: Avg or p95
   - **Group by**: `http_target`
   - **Sort**: Descending; show top 5–10.

> **Tip**: `duration` might be in nanoseconds depending on exporter. If Better Stack allows unit transforms, you can divide to show ms; otherwise annotate the chart.

#### Use your custom metrics (already in your code)

You’re emitting:

- `histogram.request_duration_ms`
- `histogram.total_time_ms`, `histogram.load_time_ms`, `histogram.clean_time_ms`, `histogram.aggregate_time_ms`, `histogram.sort_filter_time_ms`, `histogram.save_time_ms`
- `monotonic_counter.http_requests_total`
- `monotonic_counter.rows_processed`
- **Context attrs**: `route`, `stage`

**Fastest path (no schema work): query logs directly**

Dashboards → Metrics panel → “Logs” query (or Explore):

- Search for `histogram.request_duration_ms`
  - **Aggregation**: p95 (or avg)
  - **Group by**: `route`
  - **Time grain**: 1m

- Same for ETL stage timings:
  - Query `histogram.*_time_ms`
  - **Aggregation**: Avg
  - **Group by**: `stage`

**Optional: define Metrics in Better Stack for low-latency charts**

If you want sub-second dashboards (pre-computed time series), create Metrics from your log fields:

1. In Better Stack → Telemetry → Metrics → New metric
2. **Source**: Your Shuttle source
3. **Field → Type**:
   - `monotonic_counter.http_requests_total` → Counter (unit: “requests”, labels: `route`)
   - `monotonic_counter.rows_processed` → Counter (unit: “rows”)
   - `histogram.request_duration_ms` → Histogram (unit: “ms”, labels: `route`)
   - `histogram.total_time_ms` / stage timings → Histogram (unit: “ms”, labels: `stage`)
4. Save, then use them in Dashboard charts:
   - Counters → Rate or sum
   - Histograms → p50/p95/avg

Keep label cardinality low (e.g., small set of `route` or `stage` values) so queries stay fast.

**Canonical “starter” charts (copy this structure)**

- **Requests per route (rate)**:
  - **Metric**: `monotonic_counter.http_requests_total` → rate(1m) → group by `route`
- **Request latency p95 per route**:
  - **Metric**: `histogram.request_duration_ms` → p95 → group by `route`
- **ETL stage time (avg)**:
  - **Metric**: `histogram.*_time_ms` → avg → group by `stage` (stacked area or multi-line)
- **Rows processed rate**:
  - **Metric**: `monotonic_counter.rows_processed` → rate(1m)
- **Health checks**:
  - Count or rate of events where `endpoint="/health"` (from your `tracing::info!`), grouped by `outcome` if you add one.

**Sanity checklist if something doesn’t show**

- Hit the endpoints a few times:
  ```bash
  curl -s https://<your-app>.shuttle.app/benchmark >/dev/null
  ```
- In Better Stack → Telemetry → Logs: search `histogram.request_duration_ms` or `monotonic_counter.http_requests_total`. If you see them in logs, they can be charted.
- For built-ins like `duration`, search for spans with `http_target` present; use `p95(duration)`.

That’s it—you can mix the built-ins (`duration`, `span_name`, `http_target`, `error`) with your custom counters/histograms to build a complete view without running an OTEL collector yourself.

## Configuration

### Environment variables

- `DATA_PATH` — Path to the CSV for local runs; default `data/yellow_tripdata_2015-01.csv`

### Feature flags

- `bench-cli` — Builds the local CLI benchmark main.
  ```bash
  RUST_LOG=info cargo run --release --features bench-cli
  ```

- `otel-otlp` — Enables an explicit OTLP exporter (used when running outside Shuttle).
  ```bash
  OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 \
  RUST_LOG=info cargo run --release --features otel-otlp
  ```

> **Note**: On Shuttle, you do not need `otel-otlp`. The platform’s runtime exports data for you.

## Performance notes

The NYC Taxi CSV is large (~1.9GB). Expect high memory usage during local aggregation. Prefer running the full ETL locally; the Shuttle service intentionally returns representative numbers to keep the dyno responsive.

For real pipelines, consider Parquet input and column pruning with Polars’ lazy API.

## Troubleshooting

### No telemetry in Better Stack
- Confirm Better Stack is enabled in Shuttle settings with a valid token.
- Redeploy after changing settings: `cargo shuttle deploy`.
- Hit endpoints a few times; then check Telemetry → Logs for `histogram.*` or `monotonic_counter.*`.
- Ensure you deployed without the `bench-cli` feature.

### Metrics not visible in dashboard picker
- Use the exact prefixes: `histogram.`, `counter.`, `monotonic_counter.`.
- Try Explore first; dashboards may take a short time after first ingestion.
- Keep attribute cardinality in check (e.g., a small set of `route`/`stage` values).

### Data file not found (local CLI)
- Place your CSV at `./data/yellow_tripdata_2015-01.csv` or set `DATA_PATH=/path/to/file.csv`.

## Shuttle and LogFire

Today this template uses Shuttle’s OTEL → Better Stack integration. Shuttle is building towards streamlined integrations such as LogFire; keep an eye on the Shuttle site for updates:

- [Shuttle](https://www.shuttle.dev/)

## License

MIT (or your preferred license)

## Contributing

Issues and PRs welcome. If you add new ETL stages, please also add tracing events for counters/histograms so dashboards stay valuable.

## Appendix: Example responses

**GET /benchmark** (representative payload):

```json
{
  "metrics": {
    "load_time": 1.2,
    "clean_time": 0.8,
    "aggregate_time": 0.4,
    "sort_filter_time": 0.3,
    "save_time": 0.1,
    "total_time": 2.8
  },
  "message": "Demo benchmark complete (server-safe)",
  "performance_summary": "Polars processed ~12.7M taxi records in ~2.8s (demo) → ~4550000 rows/sec via Rust + Polars + OTEL."
}
```

## Links

- [Shuttle](https://www.shuttle.dev/)
- [Better Stack](https://betterstack.com/)
- [Polars](https://www.pola.rs/)
- [OpenTelemetry](https://opentelemetry.io/)
- [tracing (Rust)](https://docs.rs/tracing)
```

mod etl;
mod observability;

use axum::{routing::get, extract::Query, response::Json, Router, http::StatusCode};
use etl::PolarsETL;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

//
// =============== 1) LOCAL CLI BENCH (feature: bench-cli) ===============
// Build & run locally:  cargo run --release --features bench-cli
//
#[cfg(feature = "bench-cli")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    observability::init_tracing("polars-etl-benchmark-cli");

    println!("{}", "=".repeat(50));
    println!("🚀 STARTING POLARS ETL BENCHMARK");
    println!("{}", "=".repeat(50));

    // Check if data file exists (configurable via env)
    let data_file = std::env::var("DATA_PATH")
   	 .unwrap_or_else(|_| "data/yellow_tripdata_2015-01.csv".to_string());

    if !std::path::Path::new(&data_file).exists() {
   	 eprintln!("❌ Data file not found: {}", &data_file);
   	 eprintln!("Tip: put the file at ./data/yellow_tripdata_2015-01.csv or set DATA_PATH=<path>");
	 return Ok(());

    }


    let total_start = Instant::now();
    let mut etl = PolarsETL::new();

    match etl
        .load_data(&data_file)?
        .clean_data()?
        .aggregate_data()?
        .sort_and_filter()?
        .save_results("../results")
    {
        Ok(_) => {
            let total_time = total_start.elapsed().as_secs_f64();

            println!("\n{}", "=".repeat(50));
            println!("🎉 POLARS BENCHMARK COMPLETE!");
            println!("{}", "=".repeat(50));
            println!("⏱️  Total time: {:.2} seconds", total_time);

            println!("\n📈 Key Performance Metrics:");
            let metrics = etl.get_metrics();
            for (key, value) in metrics {
                if key.contains("time") {
                    let formatted_key = key.replace('_', " ")
                        .split_whitespace()
                        .map(|w| {
                            let mut c = w.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!("  {}: {:.2}s", formatted_key, value);
                }
            }
            println!("{}", "=".repeat(50));
        }
        Err(e) => {
            println!("❌ Error during Polars benchmark: {}", e);
        }
    }

    Ok(())
}

//
// =============== 2) SHUTTLE WEB APP (default build) ====================
// Deploy: cargo shuttle deploy
//
#[cfg(not(feature = "bench-cli"))]
#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
    let app = api_router().layer(CorsLayer::permissive()).layer(TraceLayer::new_for_http());
    tracing::info!(event = "startup", message = "Axum app ready");
    Ok(app.into())
}

#[derive(Deserialize)]
struct BenchmarkQuery {
    #[serde(default)]
    sample_size: Option<usize>,
}

#[derive(Serialize)]
struct BenchmarkResult {
    metrics: HashMap<String, f64>,
    message: String,
    performance_summary: String,
}

async fn health() -> &'static str {
// emit a cheap heartbeat counter
    tracing::info!(monotonic_counter.health_checks = 1, "health checked");
    tracing::info!(endpoint = "/health", status = "ok");
    "ok"
}

#[tracing::instrument(name = "benchmark", skip_all)]
async fn run_benchmark(_q: Query<BenchmarkQuery>) -> Result<Json<BenchmarkResult>, StatusCode> {
    let t0 = Instant::now();

// Count requests to this endpoint
    tracing::info!(monotonic_counter.http_requests_total = 1, route="/benchmark", "Benchmark hit");

    tracing::info!(endpoint = "/benchmark", event = "start_demo_benchmark");
 // NOTE: We return representative metrics here (do not run heavy ETL on the Shuttle dyno).
    let mut metrics = HashMap::new();
    metrics.insert("load_time".to_string(), 1.2);
    metrics.insert("clean_time".to_string(), 0.8);
    metrics.insert("aggregate_time".to_string(), 0.4);
    metrics.insert("sort_filter_time".to_string(), 0.3);
    metrics.insert("save_time".to_string(), 0.1);
    metrics.insert("total_time".to_string(), 2.8);

// Export timings as histograms (ms) so Better Stack can graph them
    tracing::info!(histogram.load_time_ms = 1.2_f64 * 1000.0, stage="load", "stage timing");
    tracing::info!(histogram.clean_time_ms = 0.8_f64 * 1000.0, stage="clean", "stage timing");
    tracing::info!(histogram.aggregate_time_ms = 0.4_f64 * 1000.0, stage="aggregate", "stage timing");
    tracing::info!(histogram.sort_filter_time_ms = 0.3_f64 * 1000.0, stage="sort_filter", "stage timing");
    tracing::info!(histogram.save_time_ms = 0.1_f64 * 1000.0, stage="save", "stage timing");
    tracing::info!(histogram.total_time_ms = 2.8_f64 * 1000.0, stage="total", "stage timing");

    // Rows processed (monotonic counter)
    tracing::info!(monotonic_counter.rows_processed = 12_748_986_i64, "rows processed (demo)");

    // Request duration histogram
    let elapsed_ms = t0.elapsed().as_millis() as f64;
    tracing::info!(histogram.request_duration_ms = elapsed_ms, route="/benchmark", "Benchmark responded");

    let rows_per_second = 12_748_986.0 / 2.8;
    let summary = format!(
        "🚀 Polars processed ~12.7M taxi records in ~2.8s (demo) → ~{:.0} rows/sec via Rust + Polars + OTEL.",
        rows_per_second
    );
    tracing::info!(endpoint = "/benchmark", event = "end_demo_benchmark", total_time = 2.8);

    Ok(Json(BenchmarkResult {
        metrics,
        message: "✅ Demo benchmark complete (server-safe)".into(),
        performance_summary: summary,
    }))
}

fn api_router() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/benchmark", get(run_benchmark))
}


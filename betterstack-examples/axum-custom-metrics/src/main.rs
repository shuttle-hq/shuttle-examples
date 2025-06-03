use axum::{
    routing::get, 
    Router,
    http::StatusCode,
    response::IntoResponse
};
use std::time::Instant;
use tracing::{info, error};
use sysinfo::{System, SystemExt, CpuExt};

async fn hello_world() -> impl IntoResponse {
    let start = Instant::now();

    // Traffic Signal - Increment request counter
    info!(monotonic_counter.requests_total = 1, "Request received");

    // Collect system metrics for saturation
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Saturation Signal - Record various system metrics
    let memory_used_percent = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
    info!(gauge.memory_used_percent = memory_used_percent, "Memory usage");
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    info!(gauge.cpu_usage_percent = cpu_usage, "CPU usage");

    // Simulate some work and potential error condition - this intentionaly throws an error 90% of the time
    let result: Result<&'static str, StatusCode> = if rand::random::<f32>() < 0.9 {
        Ok("Hello, world!")
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    // Latency Signal - Record request duration
    let duration = start.elapsed().as_millis() as f64;
    info!(histogram.request_duration_ms = duration, "Request completed");

    // Error Signal - Only increment error counter on actual errors
    match &result {
        Ok(_) => info!(counter.request_success = 1, "Request successful"),
        Err(_) => error!(counter.request_errors = 1, "Request failed")
    }

    // Convert the result to a response
    match result {
        Ok(message) => (StatusCode::OK, message).into_response(),
        Err(status) => status.into_response(),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world));

    Ok(router.into())
}

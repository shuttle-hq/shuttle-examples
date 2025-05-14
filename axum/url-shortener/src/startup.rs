// src/lib/startup.rs

use crate::routes::{get_redirect, health_check, post_shorten};
use crate::telemetry::MakeRequestUuid;
use axum::{
    http::HeaderName,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub struct Application(pub Router);

impl Application {
    // builds the router for the application
    pub fn build(pool: PgPool) -> Self {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));
        let x_request_id = HeaderName::from_static("x-request-id");

        // build the router, with state and tracing
        let router = Router::new()
            .route("/health_check", get(health_check))
            .route("/{id}", get(get_redirect))
            .route("/", post(post_shorten))
            .with_state(pool)
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );

        Self(router)
    }
}

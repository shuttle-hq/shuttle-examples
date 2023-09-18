use crate::AppState;
use axum::{
    http::{self},
    middleware::{self},
    routing::{get, post},
    Router,
};
use http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

use crate::auth::{login, logout, register, validate_session};
use crate::customers::{
    create_customer, destroy_customer, edit_customer, get_all_customers, get_customer_names,
    get_one_customer,
};
use crate::dashboard::get_dashboard_data;
use crate::deals::{create_deal, destroy_deal, edit_deal, get_all_deals, get_one_deal};
use crate::mail::subscribe;
use crate::payments::create_checkout;

pub fn create_api_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(state.domain.parse::<HeaderValue>().unwrap());

    let payments_router = Router::new().route("/pay", post(create_checkout));

    let customers_router = Router::new()
        .route("/", post(get_all_customers))
        .route("/names", post(get_customer_names))
        .route(
            "/:id",
            post(get_one_customer)
                .put(edit_customer)
                .delete(destroy_customer),
        )
        .route("/create", post(create_customer));

    let deals_router = Router::new()
        .route("/", post(get_all_deals))
        .route(
            "/:id",
            post(get_one_deal).put(edit_deal).delete(destroy_deal),
        )
        .route("/create", post(create_deal));

    let auth_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout));

    Router::new()
        .nest("/customers", customers_router)
        .nest("/deals", deals_router)
        .nest("/payments", payments_router)
        .route("/dashboard", post(get_dashboard_data))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            validate_session,
        ))
        .nest("/auth", auth_router)
        .route("/subscribe", post(subscribe))
        .route("/health", get(hello_world))
        .with_state(state)
        .layer(cors)
}

pub async fn hello_world() -> &'static str {
    "Hello world!"
}

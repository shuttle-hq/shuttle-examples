use axum::{routing::get, Router, extract::State};
use std::sync::Arc;


struct AppState {
    info: shuttle_dynamodb::DynamoDbReadyInfo
}

async fn hello_dynamodb(State(state): State<Arc<AppState>>) -> String{
    format!("{:?}", &state.info)
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_dynamodb::DynamoDB] info: shuttle_dynamodb::DynamoDbReadyInfo,
) -> shuttle_axum::ShuttleAxum {

    let shared_state = Arc::new(AppState { info });

    let router = Router::new().route("/", get(hello_dynamodb)).with_state(shared_state);

    Ok(router.into())
}
use axum::{routing::get, Router, extract::State};
use std::sync::Arc;
use aws_config::provider_config::ProviderConfig;
use std::env::Env;


struct AppState {
    info: shuttle_dynamodb::DynamoDbReadyInfo,
    dynamodb_client: aws_sdk_dynamodb::Client,
}

async fn hello_dynamodb(State(state): State<Arc<AppState>>) -> String{

    format!("{:?}", &state.info)
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_dynamodb::DynamoDB] info: shuttle_dynamodb::DynamoDbReadyInfo,
) -> shuttle_axum::ShuttleAxum {

    let env = Env::from_slice(&[
        ("AWS_ACCESS_KEY_ID", info.aws_access_key_id),
        ("AWS_SECRET_ACCESS_KEY", info.aws_secret_access_key),
        ("AWS_REGION", info.aws_default_region),
    ]);

    let aws_config = aws_config::from_env()
        .configure(
            ProviderConfig::empty()
                .with_env(env)
        )
        .load()
        .await;

    let dynamodb_client = aws_sdk_dynamodb::Client::new(&aws_config);
    

    let shared_state = Arc::new(AppState { info, dynamodb_client });

    let router = Router::new().route("/", get(hello_dynamodb)).with_state(shared_state);

    

    Ok(router.into())
}
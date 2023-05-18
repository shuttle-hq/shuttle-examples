use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_dynamodb::DynamoDB] info: shuttle_dynamodb::DynamoDbReadyInfo,
) -> shuttle_axum::ShuttleAxum {
    // shuttle_dynamodb::DynamoDB::new();
    // let router = Router::new().route("/", get(move || async { 
    //     format!("{:?}", info)
    //  }));

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
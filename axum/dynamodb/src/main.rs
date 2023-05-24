use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

struct AppState {
    dynamodb_client: aws_sdk_dynamodb::Client,
    prefix: String,
}

async fn hello_dynamodb(State(state): State<Arc<AppState>>) -> String {
    let a_name: String = "test".into();
    let table_name: String = "test".into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    let create_table_response = state
        .dynamodb_client
        .create_table()
        .table_name(format!("{}-test", state.prefix))
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    format!("{:?}", create_table_response)
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_dynamodb::DynamoDB] info: shuttle_dynamodb::DynamoDbReadyInfo,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var("AWS_ACCESS_KEY_ID", info.aws_access_key_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", info.aws_secret_access_key);
    std::env::set_var("AWS_REGION", info.aws_default_region);

    let aws_config = aws_config::from_env().load().await;

    let dynamodb_client = aws_sdk_dynamodb::Client::new(&aws_config);

    let shared_state = Arc::new(AppState {
        dynamodb_client,
        prefix: info.prefix,
    });

    let router = Router::new()
        .route("/", get(hello_dynamodb))
        .with_state(shared_state);

    Ok(router.into())
}

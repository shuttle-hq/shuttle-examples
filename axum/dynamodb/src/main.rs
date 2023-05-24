use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput, CreateTableError};
use aws_sdk_dynamodb::operation::delete_table::{DeleteTableOutput, DeleteTableError};
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

struct AppState {
    dynamodb_client: aws_sdk_dynamodb::Client,
    prefix: String,
}

async fn create_table(dynamodb_client: &aws_sdk_dynamodb::Client, table_name: &str, attribute_name: &str) -> Result<CreateTableOutput, SdkError<CreateTableError>> {
    let attribute_definition = AttributeDefinition::builder()
        .attribute_name(attribute_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let key_schema = KeySchemaElement::builder()
        .attribute_name(attribute_name)
        .key_type(KeyType::Hash)
        .build();

    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    let create_table_response = dynamodb_client
        .create_table()
        .table_name(table_name)
        .key_schema(key_schema)
        .attribute_definitions(attribute_definition)
        .provisioned_throughput(provisioned_throughput)
        .send()
        .await;

    create_table_response
}

async fn delete_table(dynamodb_client: &aws_sdk_dynamodb::Client, table_name: &str) -> Result<DeleteTableOutput, SdkError<DeleteTableError>> {
    let delete_table_response = dynamodb_client
        .delete_table()
        .table_name(table_name)
        .send()
        .await;

    delete_table_response
}


async fn create_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}test", state.prefix);
    let attribute_name: String = "test".into();

    match create_table(&state.dynamodb_client, &table_name, &attribute_name).await {
        Ok(_) => {
            "created table!\n".to_string()
        },
        Err(e) => {
            format!("failed to create table\n{:?}\n", e)
        } 
    }
}

async fn delete_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}test", state.prefix);

    match delete_table(&state.dynamodb_client, &table_name).await {
        Ok(_) => {
            "deleted table!\n".to_string()
        },
        Err(e) => {
            format!("failed to delete table\n{:?}\n", e)
        } 
    }
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
        .route("/create_table", get(create_table_route))
        .route("/delete_table", get(delete_table_route))
        .with_state(shared_state);

    Ok(router.into())
}


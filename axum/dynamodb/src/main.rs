use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::create_table::{CreateTableError, CreateTableOutput};
use aws_sdk_dynamodb::operation::delete_table::{DeleteTableError, DeleteTableOutput};
use aws_sdk_dynamodb::operation::put_item::{PutItemError, PutItemOutput};
use aws_sdk_dynamodb::operation::scan::{ScanError, ScanOutput};
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use std::time::SystemTime;

struct AppState {
    dynamodb_client: aws_sdk_dynamodb::Client,
    prefix: String,
}

async fn create_table(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    attribute_name: &str,
) -> Result<CreateTableOutput, SdkError<CreateTableError>> {
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

async fn delete_table(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<DeleteTableOutput, SdkError<DeleteTableError>> {
    let delete_table_response = dynamodb_client
        .delete_table()
        .table_name(table_name)
        .send()
        .await;

    delete_table_response
}

async fn insert_into_table(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    key: &str,
    value: String,
) -> Result<PutItemOutput, SdkError<PutItemError>> {
    let insert_into_table_response = dynamodb_client
        .put_item()
        .table_name(table_name)
        .item(key, aws_sdk_dynamodb::types::AttributeValue::S(value))
        .send()
        .await;

    insert_into_table_response
}

async fn select_from_table(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<ScanOutput, SdkError<ScanError>> {
    let select_from_table_response = dynamodb_client.scan().table_name(table_name).send().await;

    select_from_table_response
}

async fn create_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}-test", state.prefix);
    let attribute_name: String = "test".into();

    match create_table(&state.dynamodb_client, &table_name, &attribute_name).await {
        Ok(_) => "created table!\n".to_string(),
        Err(e) => {
            format!("failed to create table\n{:?}\n", e)
        }
    }
}

async fn delete_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}-test", state.prefix);

    match delete_table(&state.dynamodb_client, &table_name).await {
        Ok(_) => "deleted table!\n".to_string(),
        Err(e) => {
            format!("failed to delete table\n{:?}\n", e)
        }
    }
}

async fn insert_into_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}-test", state.prefix);
    let key = "test".to_string();
    let value = format!(
        "{}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    );

    match insert_into_table(&state.dynamodb_client, &table_name, &key, value).await {
        Ok(_) => "inserted into table!\n".to_string(),
        Err(e) => {
            format!("failed to insert into table\n{:?}\n", e)
        }
    }
}

async fn select_from_table_route(State(state): State<Arc<AppState>>) -> String {
    let table_name: String = format!("{}-test", state.prefix);
    let mut output = String::new();

    match select_from_table(&state.dynamodb_client, &table_name).await {
        Ok(result) => {
            output.push_str("Selected from table\n");

            if let Some(items) = result.items {
                if items.len() > 0 {
                    for item in items.iter() {
                        output.push_str(&format!("{:?}\n", item));
                    }
                } else {
                    output.push_str("no values found\n");
                }
            } else {
                output.push_str("no values found\n");
            }
        }
        Err(e) => output.push_str(&format!("failed to select from table\n{:?}\n", e)),
    }

    output
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_dynamodb::DynamoDB] info: shuttle_dynamodb::DynamoDbReadyInfo,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var("AWS_ACCESS_KEY_ID", info.aws_access_key_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", info.aws_secret_access_key);
    std::env::set_var("AWS_REGION", info.aws_default_region);

    let mut aws_config = aws_config::from_env();

    if let Some(endpoint) = info.endpoint { // needed for local run (cargo shuttle run)
        aws_config = aws_config.endpoint_url(endpoint);
    }

    let aws_config = aws_config.load().await;

    let dynamodb_client = aws_sdk_dynamodb::Client::new(&aws_config);

    let shared_state = Arc::new(AppState {
        dynamodb_client,
        prefix: info.prefix,
    });

    let router = Router::new()
        .route("/create_table", get(create_table_route))
        .route("/delete_table", get(delete_table_route))
        .route("/insert", get(insert_into_table_route))
        .route("/select", get(select_from_table_route))
        .with_state(shared_state);

    Ok(router.into())
}

#![allow(clippy::unused_async)]
use crate::errors::ApiError;
use axum::http::StatusCode;
use axum::{
    extract::State,
    routing::{delete, get, post},
    Json,
};
use loco_rs::prelude::{AppContext, Routes};

use crate::models::_entities::{subscription_tiers, user_subscriptions, users};
use loco_rs::controller::middleware;
use sea_orm::entity::prelude::*;
use sea_orm::entity::ActiveValue;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::{DeriveActiveEnum, EnumIter, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use stripe::{
    AttachPaymentMethod, CancelSubscription, CardDetailsParams, Client, CreateCustomer,
    CreatePaymentMethod, CreatePaymentMethodCardUnion, CreatePrice, CreatePriceRecurring,
    CreatePriceRecurringInterval, CreateProduct, CreateSubscription, CreateSubscriptionItems,
    Currency, Customer, IdOrCreate, PaymentMethod, PaymentMethodTypeFilter, Price, PriceId,
    Product, Subscription, SubscriptionId, UpdateSubscription, UpdateSubscriptionItems,
};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("stripe")
        .add("/get_current_tier", get(get_current_tier))
        .add("/create", post(create_subscription))
        .add("/update", post(update_subscription_tier))
        .add("/cancel", delete(cancel_subscription))
}

pub async fn create_subscription(
    State(ctx): State<AppContext>,
    auth: middleware::auth::JWTWithUser<users::Model>,
    Json(json): Json<UserSubscription>,
) -> Result<StatusCode, ApiError> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_API_KEY in env");
    let client = Client::new(secret_key);

    let customer = Customer::create(&client, json.as_create_customer()).await?;

    let payment_method = {
        let pm = PaymentMethod::create(
            &client,
            CreatePaymentMethod {
                type_: Some(PaymentMethodTypeFilter::Card),
                card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(
                    json.as_card_details_params(),
                )),
                ..Default::default()
            },
        )
        .await?;

        PaymentMethod::attach(
            &client,
            &pm.id,
            AttachPaymentMethod {
                customer: customer.id.clone(),
            },
        )
        .await?;

        pm
    };

    let price = retrieve_product(&client, &json.user_tier, &ctx.db).await?;

    let mut params = CreateSubscription::new(customer.id.clone());
    params.items = Some(vec![CreateSubscriptionItems {
        price: Some(price.id.to_string()),
        ..Default::default()
    }]);

    params.default_payment_method = Some(&payment_method.id);
    params.expand = &["items", "items.data.price.product", "schedule"];

    let subscription = Subscription::create(&client, params).await?;

    let subscription_activemodel = user_subscriptions::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        stripe_customer_id: ActiveValue::Set(customer.id.to_string()),
        stripe_subscription_id: ActiveValue::Set(subscription.id.to_string()),
        user_tier: ActiveValue::Set(json.user_tier),
        ..Default::default()
    };

    user_subscriptions::Entity::insert(subscription_activemodel)
        .exec(&ctx.db)
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserTier {
    user_tier: UserTier,
}

pub async fn update_subscription_tier(
    State(ctx): State<AppContext>,
    auth: middleware::auth::JWTWithUser<users::Model>,
    Json(new_user_tier): Json<UpdateUserTier>,
) -> Result<StatusCode, ApiError> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_API_KEY in env");
    let client = Client::new(secret_key);

    let user_subscription = user_subscriptions::Entity::find()
        .filter(user_subscriptions::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .unwrap();

    let subscription_item = Subscription::retrieve(
        &client,
        &SubscriptionId::from_str(&user_subscription.stripe_subscription_id).unwrap(),
        &["items"],
    )
    .await?
    .items;

    let subscription_item = &subscription_item.data[0];

    if new_user_tier.user_tier == user_subscription.user_tier {
        return Err(ApiError::UserTierAlreadyExists);
    }

    let new_subscription = subscription_tiers::Entity::find()
        .filter(
            Condition::any()
                .add(subscription_tiers::Column::Tier.contains(new_user_tier.user_tier.to_string()))
                .add(
                    subscription_tiers::Column::Tier
                        .contains(user_subscription.user_tier.to_string()),
                ),
        )
        .all(&ctx.db)
        .await?;

    let updated_tier: String = new_subscription
        .iter()
        .find(|x| x.tier == new_user_tier.user_tier.to_string())
        .map(|x| x.stripe_price_id.to_string())
        .unwrap();

    let _ = Subscription::update(
        &client,
        &SubscriptionId::from_str(&user_subscription.stripe_subscription_id).unwrap(),
        UpdateSubscription {
            items: Some(vec![UpdateSubscriptionItems {
                id: Some(subscription_item.id.to_string()),
                price: Some(updated_tier),
                ..Default::default()
            }]),
            ..Default::default()
        },
    )
    .await?;

    let mut updated_user_subscription = user_subscription.into_active_model();
    updated_user_subscription.user_tier = ActiveValue::Set(new_user_tier.user_tier);

    let _ = updated_user_subscription.update(&ctx.db).await?;

    Ok(StatusCode::OK)
}

pub async fn cancel_subscription(
    State(ctx): State<AppContext>,
    auth: middleware::auth::JWTWithUser<users::Model>,
) -> Result<StatusCode, ApiError> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_API_KEY in env");
    let client = Client::new(secret_key);

    let subscription = user_subscriptions::Entity::find()
        .filter(user_subscriptions::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .unwrap();

    let _ = Subscription::cancel(
        &client,
        &SubscriptionId::from_str(&subscription.stripe_subscription_id).unwrap(),
        CancelSubscription {
            cancellation_details: None,
            invoice_now: Some(true),
            prorate: Some(true),
        },
    )
    .await?;

    let subscription_to_delete = user_subscriptions::Entity::find()
        .filter(user_subscriptions::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .unwrap();

    subscription_to_delete.delete(&ctx.db).await?;

    Ok(StatusCode::OK)
}

pub async fn get_current_tier(
    State(ctx): State<AppContext>,
    auth: middleware::auth::JWTWithUser<users::Model>,
) -> Result<Json<UpdateUserTier>, ApiError> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let subscription = user_subscriptions::Entity::find()
        .filter(user_subscriptions::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .unwrap();

    Ok(Json(UpdateUserTier {
        user_tier: subscription.user_tier,
    }))
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscription {
    name: String,
    email: String,
    card_num: String,
    exp_year: i32,
    exp_month: i32,
    cvc: String,
    user_tier: UserTier,
}

impl UserSubscription {
    fn as_create_customer(&self) -> CreateCustomer {
        CreateCustomer {
            name: Some(&self.name),
            email: Some(&self.email),
            description: Some("A paying user."),
            metadata: Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )])),

            ..Default::default()
        }
    }

    fn as_card_details_params(&self) -> CardDetailsParams {
        CardDetailsParams {
            number: self.card_num.to_string(),
            exp_year: self.exp_year,
            exp_month: self.exp_month,
            cvc: Some(self.cvc.clone()),
        }
    }
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Deserialize, Debug, Serialize, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum UserTier {
    #[sea_orm(string_value = "P")]
    Pro,
    #[sea_orm(string_value = "T")]
    Team,
}

impl UserTier {
    fn get_price(&self) -> Option<i64> {
        match self {
            Self::Pro => Some(1000),
            Self::Team => Some(2500),
        }
    }
}

impl fmt::Display for UserTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pro => write!(f, "Pro"),
            Self::Team => write!(f, "Team"),
        }
    }
}

async fn retrieve_product(
    client: &Client,
    user_tier: &UserTier,
    db: &DatabaseConnection,
) -> Result<Price, ApiError> {
    let product = subscription_tiers::Entity::find()
        .filter(subscription_tiers::Column::Tier.contains(user_tier.to_string()))
        .one(db)
        .await?;

    let price = match product {
        Some(product) => {
            Price::retrieve(
                client,
                &PriceId::from_str(&product.stripe_price_id.to_string()).unwrap(),
                &["product"],
            )
            .await?
        }
        None => {
            let product = create_product_item(client, user_tier).await?;
            let price = create_product_price(client, user_tier, &product).await?;

            let tier_model = subscription_tiers::ActiveModel {
                tier: ActiveValue::Set(user_tier.to_string()),
                stripe_item_id: ActiveValue::Set(product.id.to_string()),
                stripe_price_id: ActiveValue::Set(price.id.to_string()),
                ..Default::default()
            };

            subscription_tiers::Entity::insert(tier_model)
                .exec(db)
                .await?;

            price
        }
    };

    Ok(price)
}

async fn create_product_item(client: &Client, user_tier: &UserTier) -> Result<Product, ApiError> {
    let product = {
        let mut create_product = match user_tier {
            UserTier::Pro => CreateProduct::new("Pro User Subscription"),
            UserTier::Team => CreateProduct::new("Team Subscription"),
        };

        create_product.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));

        Product::create(client, create_product).await?
    };

    Ok(product)
}

async fn create_product_price(
    client: &Client,
    user_tier: &UserTier,
    product: &Product,
) -> Result<Price, ApiError> {
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));
        create_price.unit_amount = user_tier.get_price();

        create_price.recurring = Some(CreatePriceRecurring {
            interval: CreatePriceRecurringInterval::Month,
            ..Default::default()
        });
        create_price.expand = &["product"];
        Price::create(client, create_price).await?
    };

    Ok(price)
}

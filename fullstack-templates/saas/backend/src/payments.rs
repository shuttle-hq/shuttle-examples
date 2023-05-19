use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use stripe::{
    AttachPaymentMethod, CardDetailsParams, CreateCustomer, CreatePaymentMethod,
    CreatePaymentMethodCardUnion, CreateSubscription, CreateSubscriptionItems, Customer,
    CustomerId, PaymentMethod, PaymentMethodTypeFilter, Subscription,
};

use crate::AppState;

#[derive(Deserialize, Serialize)]
pub struct PaymentInfo {
    name: String,
    email: String,
    card: String,
    expyear: i32,
    expmonth: i32,
    cvc: String,
}

pub async fn create_checkout(
    State(state): State<AppState>,
    Json(req): Json<PaymentInfo>,
) -> Result<StatusCode, StatusCode> {
    let ctx = stripe::Client::new(&state.stripe_key);

    let customer = Customer::create(
        &ctx,
        CreateCustomer {
            name: Some(&req.name),
            email: Some(&req.email),
            ..Default::default()
        },
    )
    .await
    .expect("Had an error creating customer!");

    println!("Made a customer");

    let payment_method = {
        let pm = PaymentMethod::create(
            &ctx,
            CreatePaymentMethod {
                type_: Some(PaymentMethodTypeFilter::Card),
                card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(
                    CardDetailsParams {
                        number: req.card,
                        exp_year: req.expyear,
                        exp_month: req.expmonth,
                        cvc: Some(req.cvc),
                    },
                )),
                ..Default::default()
            },
        )
        .await
        .expect("Had an error creating payment method!");

        println!("Made a payment method");

        PaymentMethod::attach(
            &ctx,
            &pm.id,
            AttachPaymentMethod {
                customer: customer.id.clone(),
            },
        )
        .await
        .expect("Had an error attaching the payment method!");

        println!("Attached a payment method");

        pm
    };

    let mut params = create_checkout_params(customer.id, state.stripe_sub_price);

    params.default_payment_method = Some(&payment_method.id);

    let Ok(_) = Subscription::create(&ctx, params).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(StatusCode::OK)
}

fn create_checkout_params(customer_id: CustomerId, price: String) -> CreateSubscription<'static> {
    let mut params = CreateSubscription::new(customer_id);
    params.items = Some(vec![CreateSubscriptionItems {
        price: Some(price),
        ..Default::default()
    }]);
    params.expand = &["items", "items.data.price.product", "schedule"];

    params
}

#![allow(clippy::unused_async)]
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::Response,
    routing::post,
};
use loco_rs::prelude::{IntoResponse, Routes};
use stripe::{Event, EventObject, EventType};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("webhooks")
        .add("/stripe", post(handle_stripe_webhook))
}

struct StripeEvent(Event);

#[axum::async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), "whsec_xxxxx")
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

async fn handle_stripe_webhook(StripeEvent(event): StripeEvent) {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                println!(
                    "Received checkout session completed webhook with id: {:?}",
                    session.id
                );
            }
        }
        EventType::AccountUpdated => {
            if let EventObject::Account(account) = event.data.object {
                println!(
                    "Received account updated webhook for account: {:?}",
                    account.id
                );
            }
        }
        EventType::CustomerSubscriptionDeleted => {
            if let EventObject::Subscription(account) = event.data.object {
                println!(
                    "Received account updated webhook for account: {:?}",
                    account.id
                );
            }
        }
        _ => {}
    }
}

use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use clerk_rs::{
    apis::users_api::User, clerk::Clerk, validators::actix::ClerkMiddleware, ClerkConfiguration,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;

struct AppState {
    client: Clerk,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let all_users = User::get_user_list(
        &state.client,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;
    
    let list_data = match all_users{
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "FAILED",
            "message": "Unable to retrieve all users",
        }));

        }
    };

    HttpResponse::Ok().json(serde_json::json!({
        "data": list_data,
        "message": "",
        "status":"SUCCESS"
    }))
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();
    let app_config = move |cfg: &mut ServiceConfig| {
        let clerk_secret_key =
            std::env::var("CLERK_SECRET_KEY").expect("Clerk Secret key is not set");
        let clerk_config = ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
        let client = Clerk::new(clerk_config.clone());

        let state = web::Data::new(AppState { client });

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers([header::AUTHORIZATION])
            .supports_credentials();

        cfg.service(
            web::resource("/users")
                .wrap(ClerkMiddleware::new(clerk_config, None, false))
                .route(web::get().to(get_users))
                .wrap(cors),
        )
        .app_data(state);
    };

    Ok(app_config.into())
}

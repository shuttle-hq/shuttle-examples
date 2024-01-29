use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use clerk_rs::{
    apis::{sessions_api::Session, users_api::User},
    clerk::Clerk,
    ClerkConfiguration,
    ClerkModels::VerifySessionRequest,
};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;

struct AppState {
    client: Clerk,
}

#[get("")]
async fn get_users(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let session_header = req.headers().get("X-CLERK_SESSION_ID");

    let session_id = match session_header {
        Some(value) => value.to_str().ok().unwrap(),
        None => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "status":"FAILED",
                "message":"No session found"
            }))
        }
    };

    let session_cookie = req.cookie("__session").map(|c| c.value().to_string());

    let token = match session_cookie {
        Some(id) => id,
        None => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "status": "FAILED",
                "message": "No session found",
            }))
        }
    };

    let is_session_verified = Session::verify_session(
        &state.client,
        session_id,
        Some(VerifySessionRequest { token: Some(token) }),
    )
    .await;

    if is_session_verified.is_err() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "FAILED",
            "message": "Invalid session",
        }));
    }

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
    if all_users.is_err() {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "FAILED",
            "message": "Unable to retrieve all users",
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "data": "",
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
        let client = Clerk::new(clerk_config);

        let state = web::Data::new(AppState { client });

        cfg.service(web::scope("/users").service(get_users))
            .app_data(state);
    };

    Ok(app_config.into())
}

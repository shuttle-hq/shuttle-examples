use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use clerk_rs::{
    apis::{sessions_api::Session, users_api::User},
    clerk::Clerk,
    validators::actix::ClerkMiddleware,
    ClerkConfiguration,
    ClerkModels::VerifySessionRequest,
};
use shuttle_actix_web::ShuttleActixWeb;

struct AppState {
    client: Clerk,
}

#[get("/users")]
async fn get_users(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let Ok(all_users) = User::get_user_list(
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
    .await
    else {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "FAILED",
            "message": "Unable to retrieve all users",
        }));
    };

    HttpResponse::Ok().json(all_users.into_iter().map(|u| u.id).collect::<Vec<_>>())
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_config = move |cfg: &mut ServiceConfig| {
        let clerk_secret_key = secrets
            .get("CLERK_SECRET_KEY")
            .expect("Clerk Secret key is not set");
        let clerk_config = ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
        let client = Clerk::new(clerk_config.clone());

        let state = web::Data::new(AppState { client });

        cfg.service(
            web::scope("/api")
                .wrap(ClerkMiddleware::new(clerk_config, None, true))
                .service(get_users),
        )
        .service(actix_files::Files::new("/", "./frontend2/dist").index_file("index.html"))
        .app_data(state);
    };

    Ok(app_config.into())
}

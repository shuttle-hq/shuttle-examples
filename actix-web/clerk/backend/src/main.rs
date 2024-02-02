use actix_web::{
    dev::ServiceRequest,
    get,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use clerk_rs::{
    apis::users_api::User,
    clerk::Clerk,
    validators::actix::{clerk_authorize, ClerkMiddleware},
    ClerkConfiguration,
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

struct AppState {
    client: Clerk,
}

// Get the full user list of everyone who has signed in to this app
#[get("/users")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
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
            "message": "Unable to retrieve all users",
        }));
    };

    HttpResponse::Ok().json(
        all_users
            .into_iter()
            .map(|u| u.into())
            .collect::<Vec<UserModel>>(),
    )
}

// Example endpoint for extracting the calling user
#[get("/users/me")]
async fn get_user(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let srv_req = ServiceRequest::from_request(req);
    let jwt = match clerk_authorize(&srv_req, &state.client, true).await {
        Ok(value) => value.1,
        Err(e) => return e,
    };

    let Ok(user) = User::get_user(&state.client, &jwt.sub).await else {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Unable to retrieve user",
        }));
    };

    HttpResponse::Ok().json(Into::<UserModel>::into(user))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_config = move |cfg: &mut ServiceConfig| {
        let clerk_secret_key = secrets
            .get("CLERK_SECRET_KEY")
            .expect("Clerk Secret key is not set");
        let clerk_config = ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
        let client = Clerk::new(clerk_config.clone());

        let state = web::Data::new(AppState { client });

        cfg.service(
            // protect the /api routes with clerk authentication
            web::scope("/api")
                .wrap(ClerkMiddleware::new(clerk_config, None, true))
                .service(get_users)
                .service(get_user),
        )
        // serve the build files from the frontend
        .service(actix_files::Files::new("/", "./frontend/dist").index_file("index.html"))
        .app_data(state);
    };

    Ok(app_config.into())
}

/// As subset of the fields in [`clerk_rs::models::user::User`]
#[derive(Serialize, Deserialize)]
struct UserModel {
    id: Option<String>,
    username: Option<Option<String>>,
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    email_addresses: Option<Vec<clerk_rs::models::EmailAddress>>,
    profile_image_url: Option<String>,
}

impl From<clerk_rs::models::user::User> for UserModel {
    fn from(value: clerk_rs::models::user::User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
            email_addresses: value.email_addresses,
            profile_image_url: value.profile_image_url,
        }
    }
}

use crate::routes::errors::ApiError;
use crate::AppState;
use axum::{
    extract::{State, Query},
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
    Extension,
    middleware::{Next}
};
use oauth2::{basic::BasicClient, AuthorizationCode, TokenResponse, reqwest::async_http_client};
use time::Duration as TimeDuration;
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use chrono::{Duration, Local};
use serde::{Deserialize};
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String
}

pub async fn google_callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = match oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await {
        Ok(res) => res,
        Err(e) => {
            error!("An error occurred while exchanging the code: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            
        }
    };

    let profile = match state.ctx.get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send().await {
        Ok(res) => res,
        Err(e) => return Err(ApiError::ReqwestError.to_error(e.to_string())),
    };

    let profile = profile.json::<UserProfile>().await.unwrap();

    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();

    let max_age = Local::now().naive_local() + Duration::seconds(secs);

    let cookie = Cookie::build("sid", token.access_token().secret().to_owned())
        .domain(".app.localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(secs))
        .finish();


     if let Err(e) = sqlx::query("INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING")
        .bind(profile.email.clone())
        .execute(&state.db)
        .await {
        error!("Error while trying to make account: {e}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error while creating user: {e}")))
    }
    
    if let Err(e) = sqlx::query("INSERT INTO sessions (user_id, session_id, expires_at) VALUES (
        (SELECT ID FROM USERS WHERE email = $1 LIMIT 1),
         $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET 
        session_id = excluded.session_id, 
        expires_at = excluded.expires_at")
        .bind(profile.email)
        .bind(token.access_token().secret().to_owned())
        .bind(max_age)
        .execute(&state.db)
        .await {
        error!("Error while trying to make session: {e}"); 
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error while creating user: {e}")))
    }

    Ok((
        jar.add(cookie),
        Redirect::to("/protected")
    ))
}

#[derive(Deserialize, sqlx::FromRow, Clone)]
pub struct UserProfile {
    email: String
}

pub async fn check_authenticated<B>(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    mut req: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let Some(cookie) = jar.get("sid").map(|cookie| cookie.value().to_owned()) else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()));
    };

    let res = match sqlx::query_as::<_, UserProfile>("SELECT 
        users.email
        FROM sessions 
        LEFT JOIN USERS ON sessions.user_id = users.id
        WHERE sessions.session_id = $1 
        LIMIT 1")
        .bind(cookie)
        .fetch_one(&state.db)
        .await {
        Ok(res) => res,
        Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
    };

    let user = UserProfile {
        email: res.email
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn protected(
    Extension(user): Extension<UserProfile>
) -> impl IntoResponse {
    (StatusCode::OK, user.email)
}

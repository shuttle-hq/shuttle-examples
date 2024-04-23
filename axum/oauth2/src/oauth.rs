use axum::{
    extract::{FromRequest, FromRequestParts, Query, Request, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use chrono::{Duration, Local};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use time::Duration as TimeDuration;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

pub async fn google_callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
) -> Result<impl IntoResponse, ApiError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await?;

    let profile = state
        .ctx
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send()
        .await?;

    let profile = profile.json::<UserProfile>().await?;

    let Some(secs) = token.expires_in() else {
        return Err(ApiError::OptionError);
    };

    let secs: i64 = secs.as_secs().try_into()?;

    let max_age = Local::now().naive_local() + Duration::try_seconds(secs).unwrap();

    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .domain(".app.localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(secs));

    sqlx::query("INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING")
        .bind(profile.email.clone())
        .execute(&state.db)
        .await?;

    sqlx::query(
        "INSERT INTO sessions (user_id, session_id, expires_at) VALUES (
        (SELECT ID FROM USERS WHERE email = $1 LIMIT 1),
         $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
        session_id = excluded.session_id,
        expires_at = excluded.expires_at",
    )
    .bind(profile.email)
    .bind(token.access_token().secret().to_owned())
    .bind(max_age)
    .execute(&state.db)
    .await?;

    Ok((jar.add(cookie), Redirect::to("/protected")))
}

#[derive(Deserialize, sqlx::FromRow, Clone)]
pub struct UserProfile {
    email: String,
}

#[axum::async_trait]
impl FromRequest<AppState> for UserProfile {
    type Rejection = ApiError;
    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let state = state.to_owned();
        let (mut parts, _body) = req.into_parts();
        let cookiejar: PrivateCookieJar =
            PrivateCookieJar::from_request_parts(&mut parts, &state).await?;

        let Some(cookie) = cookiejar.get("sid").map(|cookie| cookie.value().to_owned()) else {
            return Err(ApiError::Unauthorized);
        };

        let res = sqlx::query_as::<_, UserProfile>(
            "SELECT
        users.email
        FROM sessions
        LEFT JOIN USERS ON sessions.user_id = users.id
        WHERE sessions.session_id = $1
        LIMIT 1",
        )
        .bind(cookie)
        .fetch_one(&state.db)
        .await?;

        Ok(Self { email: res.email })
    }
}

pub async fn protected(profile: UserProfile) -> impl IntoResponse {
    (StatusCode::OK, profile.email)
}

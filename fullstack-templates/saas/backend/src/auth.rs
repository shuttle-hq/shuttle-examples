use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use serde::Deserialize;
use sqlx::Row;
use time::Duration;

use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterDetails {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginDetails {
    email: String,
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(newuser): Json<RegisterDetails>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(newuser.password, 10).unwrap();

    let query = sqlx::query("INSERT INTO users (name, email, password) values ($1, $2, $3)")
        .bind(newuser.name)
        .bind(newuser.email)
        .bind(hashed_password)
        .execute(&state.postgres);

    match query.await {
        Ok(_) => (StatusCode::CREATED, "Account created!".to_string()).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )
            .into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(login): Json<LoginDetails>,
) -> Result<(PrivateCookieJar, StatusCode), StatusCode> {
    let query = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&login.email)
        .fetch_one(&state.postgres);

    match query.await {
        Ok(res) => {
            match bcrypt::verify(login.password, res.get("password")) {
                Ok(true) => {}
                Ok(false) => return Err(StatusCode::BAD_REQUEST),
                Err(_) => return Err(StatusCode::BAD_REQUEST),
            }

            let session_id = rand::random::<u64>().to_string();

            sqlx::query("INSERT INTO sessions (session_id, user_id) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET session_id = EXCLUDED.session_id")
                .bind(&session_id)
                .bind(res.get::<i32, _>("id"))
                .execute(&state.postgres)
                .await
                .expect("Couldn't insert session :(");

            let cookie = Cookie::build(("foo", session_id))
                .secure(!cfg!(debug_assertions)) // Only send the cookie over HTTPS in production
                .same_site(SameSite::Strict)
                .http_only(true)
                .path("/")
                .max_age(Duration::WEEK)
                .build();

            Ok((jar.add(cookie), StatusCode::OK))
        }

        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn logout(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<PrivateCookieJar, StatusCode> {
    let Some(cookie) = jar.get("sessionid").map(|cookie| cookie.value().to_owned()) else {
        return Ok(jar);
    };

    let query = sqlx::query("DELETE FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .execute(&state.postgres);

    match query.await {
        Ok(_) => Ok(jar.remove(Cookie::from("foo"))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn validate_session(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> (PrivateCookieJar, Response) {
    let Some(cookie) = jar.get("foo").map(|cookie| cookie.value().to_owned()) else {
        println!("Couldn't find a cookie in the jar");
        return (
            jar,
            (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        );
    };

    let find_session =
        sqlx::query("SELECT * FROM sessions WHERE session_id = $1 AND expires > CURRENT_TIMESTAMP")
            .bind(cookie)
            .execute(&state.postgres)
            .await;

    match find_session {
        Ok(_) => (jar, next.run(request).await),
        Err(_) => (
            jar,
            (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        ),
    }
}

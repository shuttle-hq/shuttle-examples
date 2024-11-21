use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{FromRequest, FromRequestParts, State},
    http::StatusCode,
    response::IntoResponse,
    Json, RequestPartsExt,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    PrivateCookieJar,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(sqlx::FromRow)]
struct Password(i32, String);

pub async fn register(
    State(state): State<AppState>,
    Json(json): Json<AuthRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let hash = hash_password(&json.password).unwrap();

    if let Err(e) = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(&json.username)
        .bind(hash)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while registering: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn login(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    Json(json): Json<AuthRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user: Password = match sqlx::query_as("SELECT id, password FROM users WHERE username = $1")
        .bind(&json.username)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Incorrect password or username.",
            ))
        }
    };

    let parsed_hash = PasswordHash::new(&user.1).unwrap();
    if Argon2::default()
        .verify_password(&json.password.into_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Incorrect password or username.",
        ));
    };

    let cookie = Claims::new(user.0, &json.username).into_cookie();

    Ok((StatusCode::OK, jar.add(cookie)))
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)

    let hash = argon2.hash_password(password, &salt)?;

    Ok(hash.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
    username: String,
    exp: usize,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = (StatusCode, String);
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // error type is Infallible so we can unwrap here
        let jar: PrivateCookieJar = PrivateCookieJar::from_request_parts(parts, state)
            .await
            .unwrap();
        let Some(token) = jar.get("token") else {
            return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
        };

        let token = match decode::<Claims>(
            token.value_trimmed(),
            &KEYS.decoding,
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(e) => {
                println!("ERROR: Could not decode token {token}: {e}");
                return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
            }
        };

        let claims = token.claims;

        Ok(claims)
    }
}

impl Claims {
    pub fn new(user_id: i32, username: &str) -> Self {
        let exp = 2000000000;
        Self {
            user_id,
            username: username.to_string(),
            exp,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn user_id(&self) -> &i32 {
        &self.user_id
    }

    // Return a JWT value decoded as a cookie
    pub fn into_cookie<'c>(self) -> Cookie<'c> {
        let token = encode(&Header::default(), &self, &KEYS.encoding).unwrap();

        let mut cookie = Cookie::new("token", token);
        cookie.set_same_site(SameSite::Strict);
        cookie.set_path("/");
        cookie.set_secure(true);
        cookie.set_http_only(true);

        cookie
    }
}

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Hello world!".to_string());
    Keys::new(secret.as_bytes())
});

struct Keys {
    decoding: DecodingKey,
    encoding: EncodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

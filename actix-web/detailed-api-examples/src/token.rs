use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;
use sha2::{Sha256, Digest};
use rand::Rng;
use chrono::{Utc, Duration};

pub async fn generate_token(user_id: &i32, role: &str) -> String {
    let secret_key = "iusearchbtw)"; 
    let timestamp = Utc::now().timestamp(); 
    let random_bytes: [u8; 16] = rand::thread_rng().gen(); 

    let data = format!("{}:{}:{}:{:?}", user_id, role, timestamp, random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", data, secret_key)); 
    let hashed_token = hasher.finalize();

    format!("{:x}.{}.{}", hashed_token, timestamp, base64::encode(random_bytes))
}

#[derive(Debug, Deserialize)]
pub struct TokenStruct {
    token: String,
    user_id: String,
    role: String,
    requested_page:String
}

pub async fn verify_token(rdata : web::Json<TokenStruct>) -> Result<HttpResponse, Error> {
    let secret_key = "iusearchbtw)";
    let expiration_limit: i64 = 21600;

    let parts: Vec<&str> = rdata.token.split('.').collect();
    if parts.len() != 3 {
        return Ok(HttpResponse::BadRequest().json("Token Structure Isn't Valid")); 
    }

    let token_hash = parts[0];
    let timestamp: i64 = match parts[1].parse() {
        Ok(ts) => ts,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Invalid Timestamp")),
    };
    let random_bytes = match base64::decode(parts[2]) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Encryption Isn't True")),
    };

    let data = format!("{}:{}:{}:{:?}", rdata.user_id, rdata.role, timestamp, random_bytes);
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", data, secret_key));
    let expected_hash = format!("{:x}", hasher.finalize());

    if token_hash != expected_hash {
        return Ok(HttpResponse::BadRequest().json("Invalid Token"));
    }

    let current_timestamp = Utc::now().timestamp();
    if current_timestamp - timestamp > expiration_limit {
        let new_token = generate_token(&rdata.user_id.parse::<i32>().unwrap_or(0), &rdata.role).await;
        return Ok(HttpResponse::Ok().json(format!(
            "Token Expired. New Token: {}",
            new_token
        )));
    }

    // Role ayırımı yap
    match rdata.role.as_str() {
        "admin" => {
            if rdata.requested_page == "admin_page" {
                Ok(HttpResponse::Ok().json("Welcome to Admin Page"))
            } else {
                Ok(HttpResponse::Ok().json("Page Accessible"))
            }
        }
        "user" => {
            if rdata.requested_page == "admin_page" {
                Ok(HttpResponse::NotAcceptable().finish())
            } else {
                Ok(HttpResponse::Ok().json("Page Accessible"))
            }
        }
        _ => {
            Ok(HttpResponse::BadRequest().json("Invalid Role"))
        }
    }
}
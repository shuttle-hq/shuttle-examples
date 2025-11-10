use std::env;
use std::str::FromStr;
use actix_cors::Cors;
use actix_files::Files;
use actix_session::{SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{http, web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use token::verify_token;
use crate::app::*;
use actix_web::{get, web::{ServiceConfig}, HttpResponse, Error};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use libsql::Builder;
mod token;
mod app;



#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
        dotenv().ok();
        
        let db_url = secrets.get("TURSO_DATABASE_URL").unwrap();
        let auth_token = secrets.get("TURSO_AUTH_TOKEN").unwrap();

        let db = Builder::new_remote(db_url, auth_token)
            .build()
            .await
            .expect("Failed to connect to database");

        let conn = web::Data::new(db);
        
        let config = move |cfg: &mut ServiceConfig| {
            cfg.app_data(conn.clone());
            cfg.route("/api/user/create", web::post().to(create_user));
            cfg.route("/api/user/login", web::post().to(login_user));
            cfg.route("/api/news/upload", web::post().to(news_upload));
            cfg.route("/api/events/upload", web::post().to(events_upload));
            cfg.route("/api/get", web::get().to(get_data));
            cfg.route("/api/get/a", web::get().to(get_statistics));
            cfg.route("/api/check/token", web::post().to(verify_token));
        };
    
        Ok(config.into())
}

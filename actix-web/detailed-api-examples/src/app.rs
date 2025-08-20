use std::fs::File;
use std::io::Write;
use actix_multipart::form::json::{self, Json};
use actix_multipart::Multipart;
use actix_web::{http};
use actix_web::{web,HttpResponse,Error,Result};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use rand::{self, thread_rng, Rng};
use  crate::token::*;
use chrono::Utc;
#[derive(Debug, Deserialize, Serialize)]
pub struct SigninUser{
    name: String,
    surname: String,
    email: String,
    phone: i32,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser{
    email: String,
    password: String,
}


#[derive(Serialize, Deserialize)]
pub struct StatisticsPayload {
    path : String,
    pub page: Option<i32>, 
    pub limit: Option<i32>, 
}


#[derive(Debug, Serialize, Deserialize)]
struct Statistics {
    total_income: i32,
    users_today: i32,
    news_today: i32,
    events_today: i32,
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    surname: String,
    email: String,
    password: String,
    token: String,
}

#[derive(Deserialize)]
pub struct Info {
    path : String,
    id : Option<i32>
}

#[derive(Serialize)]
struct NewsItem {
    id: i32,
    imgpath: String,
    name: String,
    description: String,
}

#[derive(Serialize)]
struct NewsItemLanding {
    imgpath: String,
    name: String,
}

pub async fn create_user(
    user: web::Json<SigninUser>,
    db: web::Data<libsql::Database>,
) -> Result<HttpResponse, Error> {
    let mut ranid: i32 = rand::thread_rng().gen();
    let conn = db.connect().expect("Hello");

    // Check if user with generated ID already exists
    let query_result = conn.query("SELECT * FROM users WHERE id = ?", libsql::params![ranid]).await;
    if query_result.is_ok() {
        ranid = rand::thread_rng().gen();
    }

    let token: String = generate_token(&ranid, "user").await;
    let registertime = Utc::now().to_string();

    let result = conn.execute(
        "INSERT INTO users (id, name, surname, email, password, number, duty, token, registertime) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        libsql::params![
            ranid,
            user.name.clone(),
            user.surname.clone(),
            user.email.clone(),
            user.password.clone(),
            user.phone,
            "user",
            token,
            registertime
        ],
    ).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("User successfully registered")),
        Err(e) => {
            eprintln!("Error occurred while inserting user: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e)))
        },
    }
}

pub async fn login_user(
    user: web::Json<LoginUser>,
    db: web::Data<libsql::Database>,
) -> Result<HttpResponse, Error> {
    let conn = db.connect().expect("A");

    // Query the database for the user
    let query_result = conn.query(
        "SELECT id, name, surname, email, password, token FROM users WHERE email = ? AND password = ?",
        libsql::params![user.email.clone(), user.password.clone()]
    ).await;

    match query_result {
        Ok(mut rows) => {
            if let Some(_) = rows.next().await.expect("B") {
                Ok(HttpResponse::Ok().json("Ok"))
            } else {
                Ok(HttpResponse::BadRequest().json("Not Ok"))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json("Not Ok"))
    }
}
pub async fn news_upload(mut payload: Multipart, db: web::Data<libsql::Database>) -> Result<HttpResponse, Error> {
    let mut name = String::new();
    let mut description = String::new();
    let mut imgpath = String::new();

    let mut ranid: i32 = rand::thread_rng().gen();
    let conn = db.connect().expect("C");

    // Check if news with generated ID already exists
    let query_result = conn.query("SELECT * FROM news WHERE id = ?", libsql::params![ranid]).await;
    if query_result.is_ok() {
        ranid = rand::thread_rng().gen();
    }

    // Process multipart form data
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().to_string();
        match field_name.as_str() {
            "file" => {
                if let Some(filename) = field.content_disposition().get_filename() {
                    imgpath = format!("../Front-End/public/newsupload/{}", filename);
                    let mut f = File::create(&imgpath)?;
                    
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        f.write_all(&data)?;
                    }
                    println!("Image Saved: {}", imgpath);
                }
            }
            "name" => {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    value.extend_from_slice(&chunk?);
                }
                name = String::from_utf8(value).unwrap_or_default();
            }
            "description" => {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    value.extend_from_slice(&chunk?);
                }
                description = String::from_utf8(value).unwrap_or_default();
            }
            _ => {}
        }
    }

    imgpath = imgpath.replace("../Front-End", "");
    let registertime = Utc::now().to_string();

    let result = conn.execute(
        "INSERT INTO news (id, imgpath, name, desc, registertime) VALUES (?, ?, ?, ?, ?)",
        libsql::params![ranid, imgpath, name, description, registertime]
    ).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("News successfully registered")),
        Err(e) => {
                eprintln!("Error occurred while inserting user: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e)))
            },
        }

}

pub async fn events_upload(mut payload: Multipart, db: web::Data<libsql::Database>) -> Result<HttpResponse, Error> {
    let mut name = String::new();
    let mut description = String::new();
    let mut imgpath = String::new();

    let mut ranid: i32 = rand::thread_rng().gen();
    let conn = db.connect().expect("D");

    // Check if event with generated ID already exists
    let query_result = conn.query("SELECT * FROM events WHERE id = ?", libsql::params![ranid]).await;
    if query_result.is_ok() {
        ranid = rand::thread_rng().gen();
    }

    // Process multipart form data
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().to_string();
        match field_name.as_str() {
            "file" => {
                if let Some(filename) = field.content_disposition().get_filename() {
                    imgpath = format!("../Front-End/public/eventsupload/{}", filename);
                    let mut f = File::create(&imgpath)?;
                    
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        f.write_all(&data)?;
                    }
                    println!("Image Saved: {}", imgpath);
                }
            }
            "name" => {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    value.extend_from_slice(&chunk?);
                }
                name = String::from_utf8(value).unwrap_or_default();
            }
            "description" => {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    value.extend_from_slice(&chunk?);
                }
                description = String::from_utf8(value).unwrap_or_default();
            }
            _ => {}
        }
    }

    imgpath = imgpath.replace("../Front-End", "");
    let registertime = Utc::now().to_string();

    let result = conn.execute(
        "INSERT INTO events (id, imgpath, name, desc, registertime) VALUES (?, ?, ?, ?, ?)",
        libsql::params![ranid, imgpath, name, description, registertime]
    ).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Events successfully registered")),
        Err(e) => {
                eprintln!("Error occurred while inserting event: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e)))
            },
        }

}



pub async fn get_data (payload: web::Query<Info>, db: web::Data<libsql::Database>) -> Result<HttpResponse, Error> {
    println!("AAAAA {}", &payload.path);
    let conn = db.connect().expect(" Get Data DataBase Error .");
    if &payload.path == "api/get/news" {
        let mut query = conn.query("SELECT * FROM news", ()).await.unwrap();
        
        let mut news_items: Vec<NewsItem> = Vec::new();
        while let Some(row) = query.next().await.unwrap() {
            let news_item = NewsItem {
                id: row.get(0).unwrap(),
                imgpath: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                description: row.get(3).unwrap(),
            };
            news_items.push(news_item);
        }
        return Ok(HttpResponse::Ok().json(news_items));
    } else if &payload.path == "api/get/landingnews" {
        let mut query = conn.query("select * from news where id = 1?", libsql::params![&payload.id]).await.unwrap();

        let mut news_items: Vec<NewsItemLanding> = Vec::new();
        while let Some(row) = query.next().await.unwrap()  {
            let news_item = NewsItemLanding {
                imgpath: row.get(0).unwrap(),
                name: row.get(1).unwrap()
            };
            news_items.push(news_item);
        }
        return Ok(HttpResponse::Ok().json(news_items)); 
        
    } else if &payload.path == "api/get/events" {
        let mut query = conn.query("select * from events", ()).await.unwrap();
        let mut events_items: Vec<NewsItem> = Vec::new();
        while let Some(row) = query.next().await.unwrap() {
            let events_item: NewsItem = NewsItem {
                id: row.get(0).unwrap(),
                imgpath: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                description: row.get(3).unwrap(),
            };
            events_items.push(events_item);
        }
        return Ok(HttpResponse::Ok().json(events_items));

    } else if &payload.path == "api/get/eventslanding" {
        let mut query = conn.query("select * from events where id = 1?", libsql::params![&payload.id]).await.unwrap();
        let mut news_items: Vec<NewsItemLanding> = Vec::new();
        while let Some(row) = query.next().await.unwrap() {
            let news_item = NewsItemLanding {
                imgpath: row.get(0).unwrap(),
                name: row.get(1).unwrap()
            };
            news_items.push(news_item);
        }
        return Ok(HttpResponse::Ok().json(news_items)); 
        
    } 

    else {
        return Ok(HttpResponse::Ok().json("Not Correct"));
    }
    
}

pub async fn get_statistics(
    query_params: web::Query<StatisticsPayload>,
    db: web::Data<libsql::Database>,
) -> Result<HttpResponse, Error> {
    let conn = db.connect().expect(" Failed To Connect Statistics Databse ");
    if &query_params.path == "api/get/income" {
        let mut user_count_result = conn.query("SELECT COUNT(*) as user_count from users", ()).await.expect(" Error ");
        let mut total_user_count: i32 = 0;
        while let Some(row) = user_count_result.next().await.unwrap(){
            total_user_count = row.get(0).unwrap();
        };
        
        let total_income = total_user_count * 3;

        let mut users_today:i32 = 0;
        let mut today_users_result = conn.query("SELECT COUNT(*) as user_count_today 
           FROM users 
           WHERE DATE(SUBSTR(registertime, 1, 10)) = DATE('now')", ()).await.expect(" Error 2 ");


        while let Some(row) = today_users_result.next().await.unwrap(){
            users_today = row.get(0).unwrap();
        };
        

        let mut news_today:i32 = 0;
        let mut today_news_result = conn.query("SELECT COUNT(*) as news_count_today 
            FROM news 
            WHERE DATE(SUBSTR(registertime, 1, 10)) = DATE('now')", ()).await.expect(" Error 3 ");

        while let Some(row) = today_news_result.next().await.unwrap(){
            news_today = row.get(0).unwrap();
        };

        let mut events_today:i32 = 0;
        let mut today_events_result = conn.query("SELECT COUNT(*) as events_count_today 
           FROM events 
           WHERE DATE(SUBSTR(registertime, 1, 10)) = DATE('now')", ()).await.expect(" Error 4 ");

        while let Some(row) = today_events_result.next().await.unwrap(){
            events_today = row.get(0).unwrap();
        };

        let statistics_data = Statistics {
            total_income,
            users_today,
            news_today,
            events_today,
        };

        return Ok(HttpResponse::Ok().json(statistics_data));
    }else if query_params.path == "api/get/all/users" {
        let page = query_params.page.unwrap_or(1);  
        let limit = query_params.limit.unwrap_or(10);  
    
        let offset = (page - 1) * limit;  
    
        match conn.query("SELECT id, name, surname, email, password, token FROM users LIMIT ? OFFSET ?", libsql::params![limit, offset]).await {
            Ok(mut query_result) => {
                let mut users = Vec::new();
    
                while let Some(row) = query_result.next().await.unwrap() {
                    let user = User {
                        id: row.get(0).unwrap(),
                        name: row.get(1).unwrap(),
                        surname: row.get(2).unwrap(),
                        email: row.get(3).unwrap(),
                        password: row.get(4).unwrap(),
                        token: row.get(5).unwrap(),
                    };
                    users.push(user);
                }
    
                return Ok(HttpResponse::Ok().json(users));
            }
            Err(e) => {
                eprintln!("Database query error: {:?}", e); 
                return Ok(HttpResponse::InternalServerError().body("Failed to fetch users"));
            }
        }
    }
    else {
        return Ok(HttpResponse::Ok().finish());
    }
}

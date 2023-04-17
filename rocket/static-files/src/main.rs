extern crate rocket;

use std::path::PathBuf;
use rocket::fs::{FileServer, relative};

#[shuttle_runtime::main]
async fn rocket(#[shuttle_static_folder::StaticFolder] _static_folder: PathBuf) -> shuttle_rocket::ShuttleRocket {

    // shuttle-static-folder supplies a folder called 'static' by default.
    // put your static resources relative to your crate root
    // see https://lib.rs/crates/shuttle-static-folder

    let rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        ;

    Ok(rocket.into())
}

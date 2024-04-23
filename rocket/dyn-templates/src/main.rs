#[macro_use]
extern crate rocket;
// uncomment if you also wish to serve static resources
// use rocket::fs::{FileServer, relative};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/", hello(name = "your-name")))
}
#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["Example", "List", "Of", "Five", "Items"],
        },
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        // If you also wish to serve static content, uncomment line below and corresponding 'use' on line 4
        // .mount("/", FileServer::from(relative!("templates")))
        .mount("/", routes![index, hello])
        .attach(Template::fairing());

    Ok(rocket.into())
}

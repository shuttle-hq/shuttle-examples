#[macro_use]
extern crate rocket;
// use rocket::fs::{FileServer, relative};
use std::path::PathBuf;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/", hello(name = "Your Name")))
}
#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["Example", "List", "Of", "Five", "Items"],
    })
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_static_folder::StaticFolder(folder="templates")] static_folder: PathBuf) -> shuttle_rocket::ShuttleRocket {

    /* The provisioned static folder template directory will not be a sub folder 
       of the location of the executable so it is necessary to merge the 
       template_dir setting into the configuration at runtime so that dynamic templates work.

       Note that shuttle does not include Rocket.toml 
       so merging config is the preferred way to modify any settings
       that would otherwise be set in Rocket.toml
    */
    let template_dir = static_folder.to_str().unwrap();
    let figment = rocket::Config::figment()
	.merge(("template_dir", template_dir));
    let rocket = rocket::custom(figment)
	// .mount("/", FileServer::from(relative!("templates")))
	.mount("/", routes![index, hello])
        .attach(Template::fairing())
        ;

    Ok(rocket.into())
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    shared::hello()
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket.into())
}

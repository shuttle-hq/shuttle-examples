#[macro_use]
extern crate rocket;

use rocket::response::status::BadRequest;
use rocket::State;

use opendal::Operator;

struct MyState {
    storage: Operator,
}

#[post("/<path>", data = "<data>")]
async fn add(
    path: String,
    data: String,
    state: &State<MyState>,
) -> Result<String, BadRequest<String>> {
    let bs = data.into_bytes();
    let length = bs.len();

    state
        .storage
        .write(&path, bs)
        .await
        .map_err(|e| BadRequest(e.to_string()))?;
    Ok(format!("path {path} written {length}B data"))
}

#[get("/<path>")]
async fn retrieve(path: String, state: &State<MyState>) -> Result<String, BadRequest<String>> {
    let bs = state
        .storage
        .read(&path)
        .await
        .map_err(|e| BadRequest(e.to_string()))?;
    let data = String::from_utf8_lossy(&bs.to_vec()).to_string();
    Ok(data)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_opendal::Opendal(scheme = "memory")] storage: Operator,
) -> shuttle_rocket::ShuttleRocket {
    let state = MyState { storage };
    let rocket = rocket::build()
        .mount("/", routes![retrieve, add])
        .manage(state);

    Ok(rocket.into())
}

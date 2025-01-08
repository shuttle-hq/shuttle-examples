#[macro_use]
extern crate rocket;

use rocket::{
    http::Status,
    response::{status, Redirect},
    routes, State,
};
use url::Url;

struct AppState {
    op: opendal::Operator,
}

#[get("/<id>")]
async fn redirect(id: String, state: &State<AppState>) -> Result<Redirect, status::Custom<String>> {
    let url = state
        .op
        .read(&id.to_string())
        .await
        .map_err(|err| match err.kind() {
            opendal::ErrorKind::NotFound => status::Custom(
                Status::NotFound,
                "the requested shortened URL does not exist".into(),
            ),
            _ => status::Custom(Status::InternalServerError, "something went wrong".into()),
        })?;

    Ok(Redirect::to(String::from_utf8(url.to_vec()).unwrap()))
}

#[post("/", data = "<url>")]
async fn shorten<'r>(
    url: String,
    state: &State<AppState>,
    host: &'r rocket::http::uri::Host<'r>,
) -> Result<String, status::Custom<String>> {
    let _ = Url::parse(&url).map_err(|err| {
        status::Custom(
            Status::UnprocessableEntity,
            format!("url validation failed: {err}"),
        )
    })?;
    let id = &nanoid::nanoid!(6);

    state
        .op
        .write(&id.to_string(), url.into_bytes())
        .await
        .map_err(|_| status::Custom(Status::InternalServerError, "something went wrong".into()))?;

    Ok(format!("http://{host}/{id}\n"))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] op: opendal::Operator,
) -> shuttle_rocket::ShuttleRocket {
    let state = AppState { op };
    let rocket = rocket::build()
        .mount("/", routes![redirect, shorten])
        .manage(state);

    Ok(rocket.into())
}

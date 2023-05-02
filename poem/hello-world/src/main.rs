use poem::{get, handler, Route};
use shuttle_poem::ShuttlePoem;

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn poem() -> ShuttlePoem<impl poem::Endpoint> {
    let app = Route::new().at("/", get(hello_world));

    Ok(app.into())
}

use warp::Filter;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed().into())
}

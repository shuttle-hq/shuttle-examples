use rama::{
    Context, Layer,
    error::ErrorContext,
    http::{
        StatusCode,
        layer::forwarded::GetForwardedHeaderLayer,
        service::web::{Router, response::Result},
    },
    net::forwarded::Forwarded,
};

async fn hello_world(ctx: Context<()>) -> Result<String> {
    Ok(match ctx.get::<Forwarded>() {
        Some(forwarded) => format!(
            "Hello cloud user @ {}!",
            forwarded
                .client_ip()
                .context("missing IP information from user")
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        ),
        None => "Hello local user! Are you developing?".to_owned(),
    })
}

#[shuttle_runtime::main]
async fn main() -> Result<impl shuttle_rama::ShuttleService, shuttle_rama::ShuttleError> {
    let router = Router::new().get("/", hello_world);

    let app =
        // Shuttle sits behind a load-balancer,
        // so in case you want the real IP of the user,
        // you need to ensure this headers is handled.
        //
        // Learn more at <https://docs.shuttle.dev/docs/deployment-environment#https-traffic>
        GetForwardedHeaderLayer::x_forwarded_for().into_layer(router);

    Ok(shuttle_rama::RamaService::application(app))
}

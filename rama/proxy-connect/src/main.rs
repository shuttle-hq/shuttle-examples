use rama::{
    context::Extensions,
    http::{
        client::HttpClient,
        layer::{
            proxy_auth::ProxyAuthLayer,
            remove_header::{RemoveRequestHeaderLayer, RemoveResponseHeaderLayer},
            trace::TraceLayer,
            upgrade::{UpgradeLayer, Upgraded},
        },
        matcher::{DomainMatcher, HttpMatcher, MethodMatcher},
        response::Json,
        server::HttpServer,
        service::web::{extract::Path, match_service},
        Body, IntoResponse, Request, Response, StatusCode,
    },
    layer::HijackLayer,
    net::http::RequestContext,
    net::stream::layer::http::BodyLimitLayer,
    net::{address::Domain, user::Basic},
    rt::Executor,
    service::service_fn,
    tcp::{client::default_tcp_connect, utils::is_connection_error},
    username::{
        UsernameLabelParser, UsernameLabelState, UsernameLabels, UsernameOpaqueLabelParser,
    },
    Context, Layer, Service,
};
use serde::Deserialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc};

#[shuttle_runtime::main]
async fn main() -> Result<impl shuttle_rama::ShuttleService, shuttle_rama::ShuttleError> {
    #[derive(Deserialize)]
    /// API parameters for the lucky number endpoint
    struct APILuckyParams {
        number: u32,
    }

    let exec = Executor::new();
    let http_service = HttpServer::auto(exec).service(
        (
            TraceLayer::new_for_http(),
            // See [`ProxyAuthLayer::with_labels`] for more information,
            // e.g. can also be used to extract upstream proxy filters
            ProxyAuthLayer::new(Basic::new("john", "secret"))
                .with_labels::<(PriorityUsernameLabelParser, UsernameOpaqueLabelParser)>(),
            // example of how one might insert an API layer into their proxy
            HijackLayer::new(
                DomainMatcher::exact(Domain::from_static("example.shuttle.local")),
                Arc::new(match_service! {
                    HttpMatcher::post("/lucky/:number") => |path: Path<APILuckyParams>| async move {
                        Json(json!({
                            "lucky_number": path.number,
                        }))
                    },
                    HttpMatcher::get("/*") => |ctx: Context<()>, req: Request| async move {
                        Json(json!({
                            "method": req.method().as_str(),
                            "path": req.uri().path(),
                            "username_labels": ctx.get::<UsernameLabels>().map(|labels| &labels.0),
                            "user_priority": ctx.get::<Priority>().map(|p| match p {
                                Priority::High => "high",
                                Priority::Medium => "medium",
                                Priority::Low => "low",
                            }),
                        }))
                    },
                    _ => StatusCode::NOT_FOUND,
                }),
            ),
            UpgradeLayer::new(
                MethodMatcher::CONNECT,
                service_fn(http_connect_accept),
                service_fn(http_connect_proxy),
            ),
            RemoveResponseHeaderLayer::hop_by_hop(),
            RemoveRequestHeaderLayer::hop_by_hop(),
        )
            .layer(service_fn(http_plain_proxy)),
    );

    let tcp_service = (
        // protect the http proxy from too large bodies, both from request and response end
        BodyLimitLayer::symmetric(2 * 1024 * 1024),
    )
        .layer(http_service);

    Ok(shuttle_rama::RamaService::transport(tcp_service))
}

async fn http_connect_accept<S>(
    mut ctx: Context<S>,
    req: Request,
) -> Result<(Response, Context<S>, Request), Response>
where
    S: Clone + Send + Sync + 'static,
{
    match ctx.get_or_try_insert_with_ctx::<RequestContext, _>(|ctx| (ctx, &req).try_into()) {
        Ok(request_ctx) => tracing::info!("accept CONNECT to {}", request_ctx.authority),
        Err(err) => {
            tracing::error!(err = %err, "error extracting authority");
            return Err(StatusCode::BAD_REQUEST.into_response());
        }
    }

    Ok((StatusCode::OK.into_response(), ctx, req))
}

async fn http_connect_proxy<S>(ctx: Context<S>, mut upgraded: Upgraded) -> Result<(), Infallible>
where
    S: Clone + Send + Sync + 'static,
{
    let authority = ctx // assumption validated by `http_connect_accept`
        .get::<RequestContext>()
        .unwrap()
        .authority
        .clone();
    tracing::info!("CONNECT to {authority}");
    let (mut stream, _) = match default_tcp_connect(&ctx, authority).await {
        Ok(stream) => stream,
        Err(err) => {
            tracing::error!(error = %err, "error connecting to host");
            return Ok(());
        }
    };
    if let Err(err) = tokio::io::copy_bidirectional(&mut upgraded, &mut stream).await {
        if !is_connection_error(&err) {
            tracing::error!(error = %err, "error copying data");
        }
    }
    Ok(())
}

async fn http_plain_proxy<S>(ctx: Context<S>, req: Request) -> Result<Response, Infallible>
where
    S: Clone + Send + Sync + 'static,
{
    let client = HttpClient::default();
    match client.serve(ctx, req).await {
        Ok(resp) => Ok(resp),
        Err(err) => {
            tracing::error!(error = %err, "error in client request");
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Default)]
pub struct PriorityUsernameLabelParser {
    key_seen: bool,
    priority: Option<Priority>,
}

impl UsernameLabelParser for PriorityUsernameLabelParser {
    type Error = Infallible;

    fn parse_label(&mut self, label: &str) -> UsernameLabelState {
        let label = label.trim().to_ascii_lowercase();

        if self.key_seen {
            self.key_seen = false;
            match label.as_str() {
                "high" => self.priority = Some(Priority::High),
                "medium" => self.priority = Some(Priority::Medium),
                "low" => self.priority = Some(Priority::Low),
                _ => {
                    tracing::trace!("invalid priority username label value: {label}");
                    return UsernameLabelState::Abort;
                }
            }
        } else if label == "priority" {
            self.key_seen = true;
        }

        UsernameLabelState::Used
    }

    fn build(self, ext: &mut Extensions) -> Result<(), Self::Error> {
        ext.maybe_insert(self.priority);
        Ok(())
    }
}

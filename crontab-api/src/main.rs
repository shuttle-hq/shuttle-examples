use axum::Router;
use shuttle_secrets::SecretStore;

mod router;

pub struct CrontabService {
    router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CrontabService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router;

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select!(
            // _ = self.discord_bot.run() => {},
            _ = serve_router => {}
        );

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] _secrets: SecretStore,
) -> Result<CrontabService, shuttle_runtime::Error> {
    let router = router::build_router();

    Ok(CrontabService { router })
}

use axum::Router;
use shuttle_runtime::async_trait;

mod router;

pub struct CronService {
    router: Router,
}

#[async_trait]
impl shuttle_service::Service for CronService {
    async fn bind(
        mut self,
        addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        let router = self.router;

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select!(
            _ = serve_router => {}
        );

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn init() -> Result<CronService, shuttle_service::Error> {
    let router = router::build_router();

    Ok(CronService { router })
}

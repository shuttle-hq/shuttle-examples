use std::sync::Arc;

use axum::Router;
use shuttle_persist::PersistInstance;

mod router;

pub struct CrontabService {
    router: Router,
    persist: PersistInstance,
}

pub struct AppState {
    persist: PersistInstance,
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
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> Result<CrontabService, shuttle_runtime::Error> {
    let app_state = Arc::new(AppState {
        persist: persist.clone(),
    });
    let router = router::build_router(app_state);

    Ok(CrontabService { router, persist })
}

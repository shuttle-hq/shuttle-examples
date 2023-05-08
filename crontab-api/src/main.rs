use std::str::FromStr;
use std::sync::Arc;

use axum::Router;
use chrono::Utc;
use cron::Schedule;
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tokio::time::sleep;

mod router;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    schedule: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crontab {
    jobs: Vec<Job>,
}

pub struct CrontabService {
    router: Router,
    persist: PersistInstance,
}

impl CrontabService {
    async fn start(&self) {
        let tab: Crontab = self.persist.load("crontab").unwrap();

        for Job { schedule, url } in tab.jobs {
            let schedule = Schedule::from_str(&schedule).unwrap();
            let job = CronJob { schedule, url };

            // TODO: spawn blocking thread on tokio::runtime::handle
        }
    }
}

pub struct CronJob {
    schedule: Schedule,
    url: String,
}

impl CronJob {
    async fn start(&self) {
        while let Some(next_run) = self.schedule.upcoming(Utc).next() {
            let next_run_in = next_run
                .signed_duration_since(chrono::offset::Utc::now())
                .to_std()
                .unwrap();
            sleep(next_run_in).await;

            // TODO: Use reqwest to call url.
        }
    }
}

pub struct AppState {
    persist: PersistInstance,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CrontabService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router;

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        // TODO: Use a channel (?) to be notified about updates to the persisted crontab & rerun `CrontabService.start()`
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
    // TODO: `persist` should probably be a channel so `CrontabService` can listen for updates and rerun its `start` method?
    let app_state = Arc::new(AppState {
        persist: persist.clone(),
    });
    let router = router::build_router(app_state);

    Ok(CrontabService { router, persist })
}

use chrono::Utc;
use std::{future::Future, str::FromStr};

use cron::Schedule;
use shuttle_runtime::async_trait;

pub struct CronService<F> {
    schedule: Schedule,
    job: fn() -> F,
}

impl<F: Future> CronService<F> {
    async fn start(&self) {
        for time in self.schedule.upcoming(Utc).take(4) {
            let now = chrono::offset::Utc::now();
            let dur = time.signed_duration_since(now);

            let now_tok = tokio::time::Instant::now();
            if let Some(instant) = now_tok.checked_add(dur.to_std().unwrap()) {
                tokio::time::sleep_until(instant).await;
                (self.job)().await;
            }
        }
    }
}

#[async_trait]
impl<F> shuttle_service::Service for CronService<F>
where
    F: Future + Send + Sync + 'static,
{
    async fn bind(
        mut self,
        _addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        self.start().await;

        println!("All done.");

        Ok(())
    }
}

async fn my_job() {
    let now = chrono::offset::Utc::now();
    println!("It is {}", now.format("%Y-%m-%d %H:%M:%S"));
}

#[shuttle_runtime::main]
async fn init(
) -> Result<CronService<impl Future<Output = ()> + Send + 'static>, shuttle_service::Error> {
    let schedule = Schedule::from_str("*/1 * * * * *").unwrap();
    Ok(CronService {
        schedule,
        job: my_job,
    })
}

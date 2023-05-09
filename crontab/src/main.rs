use std::{future::Future, str::FromStr};

use cron::Schedule;
use shuttle_crontab::CronService;
use shuttle_runtime::tracing::info;

// "Run every 2 seconds"
const SCHEDULE: &str = "*/2 * * * * *";

// The function that will be run.
async fn my_job() {
    let now = chrono::offset::Utc::now();
    info!("It is {}", now.format("%Y-%m-%d %H:%M:%S"));
}

#[shuttle_runtime::main]
async fn init(
) -> Result<CronService<impl Future<Output = ()> + Send + 'static>, shuttle_service::Error> {
    let schedule = Schedule::from_str(SCHEDULE).unwrap();
    Ok(CronService {
        schedule,
        job: my_job,
    })
}

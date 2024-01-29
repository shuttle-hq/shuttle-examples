use std::str::FromStr;
use std::sync::Arc;

use axum::Router;
use chrono::Utc;
use cron::Schedule;
use router::make_router;
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    sync::oneshot,
    time::sleep,
};
use tracing::{debug, info};

mod error;
use error::CrontabServiceError;

mod router;

pub type ShuttleCrontab = Result<CrontabService, shuttle_runtime::Error>;

type Responder<T> = oneshot::Sender<Result<T, CrontabServiceError>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawJob {
    schedule: String,
    url: String,
}

#[derive(Debug)]
enum Msg {
    NewJob(RawJob, Responder<()>),
}

#[derive(Debug, Serialize, Deserialize)]
struct Crontab {
    jobs: Vec<RawJob>,
}

#[derive(Debug)]
struct CronJob {
    schedule: Schedule,
    url: String,
}

impl CronJob {
    async fn run(&self) {
        debug!("Running job for: {}", self.url);
        while let Some(next_run) = self.schedule.upcoming(Utc).next() {
            let next_run_in = next_run
                .signed_duration_since(chrono::offset::Utc::now())
                .to_std()
                .unwrap();
            sleep(next_run_in).await;

            let res = reqwest::get(self.url.clone()).await.unwrap();
            info!("Called {} with response {}", self.url, res.status());
        }
    }
}

impl From<&RawJob> for CronJob {
    fn from(raw: &RawJob) -> Self {
        let schedule = Schedule::from_str(&raw.schedule).expect("Failed to parse schedule");
        Self {
            schedule,
            url: raw.url.clone(),
        }
    }
}

struct CronRunner {
    persist: PersistInstance,
    receiver: Receiver<Msg>,
}

impl CronRunner {
    async fn run_jobs(&mut self) {
        if let Ok(tab) = self.persist.load::<Crontab>("crontab") {
            debug!("Found {} jobs", tab.jobs.len());
            for raw in tab.jobs {
                debug!("Starting job: {:?}", raw);
                let job = CronJob::from(&raw);

                tokio::spawn(async move {
                    job.run().await;
                });
            }
        } else {
            info!("Didn't find any jobs. POST to /crontab/set to create one.");
        }

        while let Some(msg) = self.receiver.recv().await {
            let (raw, resp) = match msg {
                Msg::NewJob(raw, resp) => (raw, resp),
            };
            debug!("Channel received: {:?}", raw);

            let mut crontab = match self.persist.load::<Crontab>("crontab") {
                Ok(tab) => tab,
                Err(_) => Crontab { jobs: vec![] },
            };

            let job = CronJob::from(&raw);

            crontab.jobs.push(raw);

            debug!("Persisting {:?} jobs", crontab.jobs.len());
            let res = self.persist.save("crontab", crontab).map_err(From::from);
            let _ = resp.send(res);

            tokio::spawn(async move {
                job.run().await;
            });
        }
    }
}

pub struct CrontabService {
    router: Router,
    runner: CronRunner,
}

struct CrontabServiceState {
    sender: Sender<Msg>,
}

impl CrontabService {
    pub fn new(
        persist: PersistInstance,
        user_router: Router,
    ) -> Result<CrontabService, shuttle_runtime::Error> {
        let (sender, receiver) = mpsc::channel(32);

        let cron_runner = CronRunner { persist, receiver };
        let cron_state = Arc::new(CrontabServiceState { sender });
        let cron_router = make_router(cron_state);

        let router = user_router.nest("/crontab", cron_router);

        Ok(Self {
            router,
            runner: cron_runner,
        })
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CrontabService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router;
        let mut runner = self.runner;

        let server = async move {
            axum::serve(
                shuttle_runtime::tokio::net::TcpListener::bind(addr)
                    .await
                    .unwrap(),
                router,
            )
            .await
        };

        let (_runner_hdl, _axum_hdl) = tokio::join!(runner.run_jobs(), server);

        Ok(())
    }
}

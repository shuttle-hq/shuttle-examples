use apalis::cron::CronStream;
use apalis::cron::Schedule;
use apalis::layers::{DefaultRetryPolicy, Extension, RetryLayer};
use apalis::postgres::PostgresStorage;
use apalis::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::str::FromStr;
use tower::ServiceBuilder;

#[derive(Clone)]
struct CronjobData {
    message: String,
}
impl CronjobData {
    fn execute(&self, _item: Reminder) {
        println!("{} from CronjobData::execute()!", &self.message);
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Reminder(DateTime<Utc>);
impl From<DateTime<Utc>> for Reminder {
    fn from(t: DateTime<Utc>) -> Self {
        Reminder(t)
    }
}

impl Job for Reminder {
    const NAME: &'static str = "reminder::DailyReminder";
}

async fn say_hello_world(job: Reminder, ctx: JobContext) {
    println!("Hello world from send_reminder()!");
    // this lets you use variables stored in the CronjobData struct
    let svc = ctx.data_opt::<CronjobData>().unwrap();
    // this executes CronjobData::execute()
    svc.execute(job);
}

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_shared_db::Postgres] conn_string: String,
) -> Result<MyService, shuttle_runtime::Error> {
    let db = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(5)
        .connect(&conn_string)
        .await
        .unwrap();

    Ok(MyService { db })
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct MyService {
    db: PgPool,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let storage = PostgresStorage::new(self.db);
        // set up storage
        storage.setup().await.expect("Unable to run migrations :(");

        let schedule = Schedule::from_str("* * * * * *").expect("Couldn't start the scheduler!");

        let cron_service_ext = CronjobData {
            message: "Hello world".to_string(),
        };

        // create a servicebuilder for the cronjob
        let service = ServiceBuilder::new()
            .layer(RetryLayer::new(DefaultRetryPolicy))
            .layer(Extension(cron_service_ext))
            .service(job_fn(say_hello_world));

        // create a worker that uses the service created from the cronjob
        let worker = WorkerBuilder::new("morning-cereal")
            .with_storage(storage.clone())
            .stream(
                CronStream::new(schedule.clone())
                    .timer(apalis::prelude::timer::TokioTimer)
                    .to_stream(),
            )
            .build(service.clone());

        // start your worker up
        Monitor::new()
            .register(worker)
            .run()
            .await
            .expect("Unable to start worker");

        Ok(())
    }
}

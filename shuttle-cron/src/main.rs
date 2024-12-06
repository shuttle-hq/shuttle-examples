use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_cron::CronStream;
use apalis_cron::Schedule;
use apalis_sql::postgres::PostgresStorage;
use apalis_sql::Config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::str::FromStr;

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

async fn say_hello_world(job: Reminder, svc: Data<CronjobData>) {
    println!("Hello world from send_reminder()!");
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
        // set up storage
        PostgresStorage::setup(&self.db)
            .await
            .expect("Unable to run migrations :(");

        let config = Config::new("reminder::DailyReminder");
        let storage = PostgresStorage::new_with_config(self.db, config);

        let schedule = Schedule::from_str("* * * * * *").expect("Couldn't start the scheduler!");

        let cron_service_ext = CronjobData {
            message: "Hello world".to_string(),
        };

        let persisted_cron = CronStream::new(schedule).pipe_to_storage(storage);

        // create a worker that uses the service created from the cronjob
        let worker = WorkerBuilder::new("morning-cereal")
            .data(cron_service_ext)
            .retry(RetryPolicy::retries(5))
            .backend(persisted_cron)
            .build_fn(say_hello_world);

        // start your worker up
        worker.run().await;

        Ok(())
    }
}

use shuttle_crontab::{CrontabService, ShuttleCrontab};
use shuttle_persist::{Persist, PersistInstance};

#[shuttle_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> ShuttleCrontab {
    CrontabService::new(persist)
}

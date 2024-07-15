use std::time::Duration;

use cja::cron::{CronRegistry, Worker};

use crate::{jobs, AppState};

fn hours(h: u64) -> Duration {
    Duration::from_secs(h * 60 * 60)
}

fn cron_registry() -> CronRegistry<AppState> {
    let mut registry = CronRegistry::new();

    registry.register_job(jobs::sessions::Cleanup, hours(1));

    registry
}

pub(crate) async fn run_cron(app_state: AppState) -> cja::Result<()> {
    Ok(Worker::new(app_state, cron_registry()).run().await?)
}

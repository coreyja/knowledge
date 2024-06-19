use std::time::Duration;

use cja::cron::{CronRegistry, Worker};

use crate::{jobs::HelloJob, AppState};

fn cron_registry() -> CronRegistry<AppState> {
    let mut registry = CronRegistry::new();

    registry.register_job(HelloJob, Duration::from_secs(1));

    registry
}

pub(crate) async fn run_cron(app_state: AppState) -> miette::Result<()> {
    Ok(Worker::new(app_state, cron_registry()).run().await?)
}

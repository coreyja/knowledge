use cja::cron::*;

use crate::AppState;

fn cron_registry() -> CronRegistry<AppState> {
    CronRegistry::new()
}

pub(crate) async fn run_cron(app_state: AppState) -> miette::Result<()> {
    Worker::new(app_state, cron_registry()).run().await?;

    Ok(())
}

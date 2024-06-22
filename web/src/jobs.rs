use cja::jobs::Job;

use crate::AppState;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct HelloJob;

#[async_trait::async_trait]
impl Job<AppState> for HelloJob {
    const NAME: &'static str = "HelloJob";

    async fn run(&self, _app_state: AppState) -> miette::Result<()> {
        println!("Hello, world!");
        Ok(())
    }
}

cja::impl_job_registry!(AppState, HelloJob);

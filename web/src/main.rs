mod cron;
mod jobs;

use cja::app_state::{self, AppState as AS};
use cron::run_cron;
use db::setup_db_pool;
use miette::IntoDiagnostic;
use tracing::info;

#[derive(Clone, Debug)]
struct AppState {
    db: sqlx::PgPool,
    cookie_key: cja::server::cookies::CookieKey,
}

impl AS for AppState {
    fn db(&self) -> &sqlx::PgPool {
        &self.db
    }
    
    fn version(&self) -> &str {
        "dev"
    }
    
    fn cookie_key(&self) -> &cja::server::cookies::CookieKey {
        &self.cookie_key
    }
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let db_pool = setup_db_pool().await.unwrap(); // Fix this unwrap
    let cookie_key = cja::server::cookies::CookieKey::from_env_or_generate().unwrap();

    let app_state = AppState {
        db: db_pool,
        cookie_key,
    };

    info!("Spawning Tasks");
    let mut futures = vec![
        // tokio::spawn(run_server(routes(app_state.clone()))),
        tokio::spawn(cja::jobs::worker::job_worker(app_state.clone(), jobs::Jobs)),
    ];
    if std::env::var("CRON_DISABLED").unwrap_or_else(|_| "false".to_string()) != "true" {
        info!("Cron Enabled");
        futures.push(tokio::spawn(cron::run_cron(app_state.clone())));
    }
    info!("Tasks Spawned");

    futures::future::try_join_all(futures).await.into_diagnostic()?;

    Ok(())
}

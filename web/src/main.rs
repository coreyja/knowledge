mod cron;
mod jobs;

use cja::app_state::{self, AppState as AS};
use cron::run_cron;
use db::setup_db_pool;
use miette::IntoDiagnostic;

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

    run_cron(app_state).await.unwrap();

    Ok(())
}

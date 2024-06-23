mod cron;
mod err;
mod jobs;
mod pages;
mod routes;
mod sessions;
mod templates;
mod users;

use cja::{app_state::AppState as AS, server::run_server};
use db::setup_db_pool;
use miette::IntoDiagnostic;

use tracing::info;

type WebResult<T, E = err::Error> = Result<T, E>;

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

fn main() -> miette::Result<()> {
    let _sentry_guard = cja::setup::setup_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .into_diagnostic()?
        .block_on(async { _main().await })
}

async fn _main() -> miette::Result<()> {
    cja::setup::setup_tracing("knowledge")?;

    let db_pool = setup_db_pool()
        .await
        .map_err(|e| miette::miette!("Setup DB Failed: {e}"))?;

    let cookie_key = cja::server::cookies::CookieKey::from_env_or_generate().into_diagnostic()?;

    let app_state = AppState {
        db: db_pool,
        cookie_key,
    };

    let app = routes::routes(app_state.clone());

    info!("Spawning Tasks");
    let mut futures = vec![
        tokio::spawn(run_server(app)),
        tokio::spawn(cja::jobs::worker::job_worker(app_state.clone(), jobs::Jobs)),
    ];
    if std::env::var("CRON_DISABLED").unwrap_or_else(|_| "false".to_string()) != "true" {
        info!("Cron Enabled");
        futures.push(tokio::spawn(cron::run_cron(app_state.clone())));
    }
    info!("Tasks Spawned");

    futures::future::try_join_all(futures)
        .await
        .into_diagnostic()?;

    Ok(())
}

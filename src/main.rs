use axum::Router;
use miette::{IntoDiagnostic, Result};

use cja::{
    app_state,
    server::run_server,
    setup::{setup_sentry, setup_tracing},
    sqlx::{self, postgres::PgPoolOptions, PgPool},
};

mod cron;
mod jobs;

mod routes;

fn main() -> miette::Result<()> {
    let _sentry_guard = setup_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .into_diagnostic()?
        .block_on(async { _main().await })
}

async fn _main() -> miette::Result<()> {
    setup_tracing("knowledge")?;

    let app_state = AppState::from_env().await?;

    tracing::info!("Spawning Tasks");
    let mut futures = vec![
        tokio::spawn(run_server(routes::routes().with_state(app_state.clone()))),
        tokio::spawn(cja::jobs::worker::job_worker(app_state.clone(), jobs::Jobs)),
    ];
    if std::env::var("CRON_DISABLED").unwrap_or_else(|_| "true".to_string()) != "true" {
        futures.push(tokio::spawn(cron::run_cron(app_state.clone())));
    }
    tracing::info!("Tasks Spawned");

    futures::future::try_join_all(futures)
        .await
        .into_diagnostic()?;

    Ok(())
}

#[derive(Clone, Debug)]
struct AppState {
    db: sqlx::Pool<sqlx::Postgres>,
    cookie_key: cja::server::cookies::CookieKey,

    s3_config: S3Config,
}

#[derive(Clone, Debug)]
struct S3Config {
    region: String,
    bucket_name: String,
    endpoint_url: String,
    access_key_id: String,
    secret_access_key: String,
}

impl S3Config {
    pub fn from_env() -> miette::Result<S3Config> {
        Ok(S3Config {
            region: std::env::var("AWS_S3_REGION").into_diagnostic()?,
            bucket_name: std::env::var("AWS_S3_BUCKET").into_diagnostic()?,
            endpoint_url: std::env::var("AWS_S3_ENDPOINT").into_diagnostic()?,
            access_key_id: std::env::var("AWS_ACCESS_KEY_ID").into_diagnostic()?,
            secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY").into_diagnostic()?,
        })
    }
}

impl cja::app_state::AppState for AppState {
    fn version(&self) -> &str {
        env!("VERGEN_GIT_SHA")
    }

    fn db(&self) -> &sqlx::PgPool {
        &self.db
    }

    fn cookie_key(&self) -> &cja::server::cookies::CookieKey {
        &self.cookie_key
    }
}

impl AppState {
    pub async fn from_env() -> Result<Self> {
        let pool = setup_db_pool().await.unwrap();

        let cookie_key =
            cja::server::cookies::CookieKey::from_env_or_generate().into_diagnostic()?;

        let s3_config = S3Config::from_env()?;

        Ok(Self {
            db: pool,
            cookie_key,
            s3_config,
        })
    }
}

#[tracing::instrument(err)]
pub async fn setup_db_pool() -> miette::Result<PgPool> {
    const MIGRATION_LOCK_ID: i64 = 0xDB_DB_DB_DB_DB_DB_DB;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .into_diagnostic()?;

    sqlx::query!("SELECT pg_advisory_lock($1)", MIGRATION_LOCK_ID)
        .execute(&pool)
        .await
        .into_diagnostic()?;

    sqlx::migrate!().run(&pool).await.into_diagnostic()?;

    let unlock_result = sqlx::query!("SELECT pg_advisory_unlock($1)", MIGRATION_LOCK_ID)
        .fetch_one(&pool)
        .await
        .into_diagnostic()?
        .pg_advisory_unlock;

    match unlock_result {
        Some(b) => {
            if b {
                tracing::info!("Migration lock unlocked");
            } else {
                tracing::info!("Failed to unlock migration lock");
            }
        }
        None => panic!("Failed to unlock migration lock"),
    }

    Ok(pool)
}

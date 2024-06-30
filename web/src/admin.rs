use axum::{extract::State, routing::get};
use cja::app_state::AppState as _;
use color_eyre::eyre::Context;

use crate::{AppState, WebResult};

async fn simple_error() -> WebResult<()> {
    Err(color_eyre::eyre::eyre!("This is a test error"))?
}

async fn sql_error(State(app_state): State<AppState>) -> WebResult<()> {
    let _ = sqlx::query("This is not valid sql")
        .fetch_one(app_state.db())
        .await
        .wrap_err("We meant for this to fail but lets see the error chain")?;
    Ok(())
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/test_errors/simple", get(simple_error))
        .route("/test_errors/sql", get(sql_error))
}

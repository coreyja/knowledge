// File: add_url.rs
use color_eyre::Result;
use db::PgPool;
use uuid::Uuid;

use crate::check_auth_status;

#[derive(Debug)]
pub enum AddUrlOutcome {
    Created(String),
    Existing(String),
}

pub async fn add_url(
    pool: &PgPool,
    url: &str,
    user_id: Uuid,
    allow_existing: bool,
) -> Result<AddUrlOutcome> {
    check_auth_status(pool).await?;

    let result = db::add_url(pool, url, user_id, &allow_existing).await;
    match result {
        Ok(page) if page.url.contains("re-adding is allowed") => {
            Ok(AddUrlOutcome::Existing(page.url.to_string()))
        }
        Ok(page) => Ok(AddUrlOutcome::Created(page.url.to_string())),
        Err(e) => Err(e),
    }
}

use color_eyre::Result;
use db::add_urls::{add_url, AddUrlOutcome, PgPool};
use uuid::Uuid;

use crate::check_auth_status;
pub async fn add_url_to_db(
    pool: &PgPool,
    url: &str,
    user_id: Uuid,
    allow_existing: bool,
) -> Result<AddUrlOutcome> {
    check_auth_status(pool).await?;
    add_url(pool, url, user_id, &allow_existing).await
}

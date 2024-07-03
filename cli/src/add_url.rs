use color_eyre::Result;
use cores::urls::{add_url, AddUrlOutcome, PgPool};
use uuid::Uuid;

use crate::check_auth_status;
pub async fn append_url(
    pool: &PgPool,
    url: &str,
    user_id: Uuid,
    allow_existing: bool,
) -> Result<AddUrlOutcome> {
    check_auth_status(pool).await?;
    add_url(pool, url, user_id, &allow_existing).await
}

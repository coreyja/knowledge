// File: add_url.rs
use color_eyre::Result;
use db::PgPool;
use uuid::Uuid;

use crate::check_auth_status;

pub async fn add_url(pool: &PgPool, url: &str, user_id: Uuid, allow_existing: bool) -> Result<()> {
    check_auth_status(pool).await?;

    let result = db::add_url(pool, url, user_id, &allow_existing).await;
    match result {
        Ok(page) if page.url.contains("re-adding is allowed") => println!("{:?}", page),
        Ok(page) => println!("URL added successfully with ID: {:?}", page),
        Err(e) => println!("Error: {:?}", e),
    }
    Ok(())
}

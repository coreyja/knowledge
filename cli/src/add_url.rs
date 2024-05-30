// File: add_url.rs
use db::PgPool;
use color_eyre::Result;

use crate::check_auth_status;

pub async fn add_url(pool: &PgPool, url: &str, allow_existing: bool) -> Result<()> {

    check_auth_status(pool).await?;

    let result = db::add_url(pool, url, &allow_existing).await;
    match result {
        Ok(message) if message.contains("re-adding is allowed") => println!("{message}"),
        Ok(message) => println!("URL added successfully with ID: {message}"),
        Err(e) => println!("Error: {e}"),
    }
    Ok(())
}
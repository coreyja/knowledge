// File: add_url.rs
use db::PgPool;
use color_eyre::Result;

use crate::check_auth_status;

pub async fn add_url(pool: &PgPool, url: &str) -> Result<()> {

    check_auth_status(pool).await?;

    let result = db::add_url(pool, url).await?;
    println!("URL added successfully with ID: {}", result);
    Ok(())
}
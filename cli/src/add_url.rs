// File: add_url.rs
use db::PgPool;
use std::fs;
use uuid::Uuid;


pub async fn add_url(pool: &PgPool, url: &str) -> color_eyre::Result<()> {
    let result = db::add_url(pool, url).await?;
    println!("URL added successfully with ID: {}", result);
    Ok(())
}
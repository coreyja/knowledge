// File: add_url.rs
use db::PgPool;
use color_eyre::Result;

use crate::check_auth_status;

pub async fn add_url(pool: &PgPool, url: &str, allow_existing: bool) -> Result<()> {

    check_auth_status(pool).await?;

    let result = db::add_url(pool, url, &allow_existing).await;
    match result {
        Ok(url_id) => println!("URL added successfully with ID: {}", url_id),
        Err(e) => {
            if allow_existing && e.to_string().contains("already saved") {
                println!("URL already exists, but will not error out due to --allow-existing flag.");
            } else {
                println!("Error: {}", e);
            }
        }, 
    }
    Ok(())
}
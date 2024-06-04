pub use sqlx;
use sqlx::types::chrono;
pub use sqlx::PgPool;
use uuid::Uuid;
use color_eyre::Result;
use reqwest;

#[derive(sqlx::FromRow, Debug)]
pub struct Page {
    pub page_id: Uuid,
    pub user_id: Uuid,
    pub url: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum AddUrlOutcome {
    Created(Page),
    Existing(Page),
}

pub async fn add_url(
    pool: &PgPool,
    url: &str,
    user_id: Uuid,
    allow_existing: &bool,
) -> color_eyre::Result<AddUrlOutcome> {
    let new_page_id = Uuid::new_v4();

    let upsert_result = sqlx::query_as!(
        Page,
        "INSERT INTO Pages (page_id, user_id, url) VALUES ($1, $2, $3)
         ON CONFLICT (user_id, url) DO UPDATE SET url = EXCLUDED.url
         RETURNING *",
        new_page_id,
        user_id,
        url
    )
    .fetch_one(pool)
    .await?;

    if upsert_result.page_id == new_page_id {
        println!("URL '{}' added successfully.", upsert_result.url);
        Ok(AddUrlOutcome::Created(upsert_result))
    } else if *allow_existing {
        println!(
            "URL '{}' exists but re-adding is allowed.",
            upsert_result.url
        );
        Ok(AddUrlOutcome::Existing(upsert_result))
    } else {
        Err(color_eyre::eyre::eyre!(
            "URL '{}' already exists and re-adding is not allowed.",
            upsert_result.url
        ))
    }
}


pub async fn fetch_url_from_pages_table(pool: &PgPool, page_id: Uuid) -> color_eyre::Result<Page> {
    let result = sqlx::query_as!(
        Page,
        "SELECT page_id, user_id, url FROM Pages WHERE page_id = $1",
        page_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}


async fn download_raw_html(pool: &PgPool, page_id: Uuid) -> color_eyre::Result<String> {
    let page = fetch_url_from_pages_table(pool, page_id).await?;
    let response = reqwest::get(&page.url).await.map_err(|e| color_eyre::Report::from(e))?;
    let html = response.text().await.map_err(|e| color_eyre::Report::from(e))?;
    Ok(html)
}

async fn store_raw_html_in_page_snapshot(pool: &PgPool, page_id: Uuid) -> color_eyre::Result<()> {
    let raw_html = download_raw_html(pool, page_id).await?;
    let current_time = chrono::Utc::now();
    let page = fetch_url_from_pages_table(pool, page_id).await?;
    
    let result = sqlx::query!(
        "INSERT INTO PageSnapShot (raw_html, fetched_at, page_id) 
         VALUES ($1, $2, $3)",
         raw_html,
         current_time, 
         page.page_id
    )
    .execute(pool)
    .await?;



    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(color_eyre::eyre::eyre!(
            "Failed to update PageSnapShot for page_id '{}'.",
            page_id, 
        ))
    }
}

pub async fn process_page_snapshot(pool: &PgPool, page_id: Uuid, user_id: Uuid, url: &str) -> color_eyre::Result<()> {
    let outcome = store_raw_html_in_page_snapshot(pool, page_id).await?;
    println!("Outcome: {:?}", outcome);
    Ok(())
}

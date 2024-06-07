use readability;
use readability::extractor;
use reqwest;
pub use sqlx;
use sqlx::types::chrono;
pub use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

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

#[derive(sqlx::FromRow, Debug)]
pub struct PageSnapShot {
    pub page_snapshot_id: Uuid,
    pub page_id: Uuid,
    pub raw_html: String,
    pub cleaned_html: String,
    pub fetched_at: Option<chrono::DateTime<chrono::Utc>>,
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

async fn download_raw_html(url: &str) -> color_eyre::Result<String> {
    let response = reqwest::get(url).await.map_err(color_eyre::Report::from)?;
    let html = response.text().await.map_err(color_eyre::Report::from)?;
    Ok(html)
}

fn clean_raw_html(raw_html: &str, url: &Url) -> color_eyre::Result<String> {
    let mut raw_html_cursor = std::io::Cursor::new(raw_html);
    let article = extractor::extract(&mut raw_html_cursor, url)
        .map_err(|e| color_eyre::eyre::eyre!(e.to_string()))?;
    Ok(article.content)
}

async fn store_raw_html_in_page_snapshot(
    pool: &PgPool,
    page: Page,
) -> color_eyre::Result<PageSnapShot> {
    let raw_html = download_raw_html(&page.url).await?;
    let current_time = chrono::Utc::now();
    let url = Url::parse(&page.url)?;
    let cleaned_html = clean_raw_html(&raw_html, &url)?;

    let result = sqlx::query_as!(
        PageSnapShot,
        "INSERT INTO PageSnapShot (raw_html, fetched_at, cleaned_html, page_id) 
        VALUES ($1, $2, $3, $4) RETURNING *",
        raw_html,
        current_time,
        cleaned_html,
        page.page_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn process_page_snapshot(pool: &PgPool, page: Page) -> color_eyre::Result<()> {
    let outcome = store_raw_html_in_page_snapshot(pool, page).await?;
    println!("Outcome: {outcome:?}");
    Ok(())
}

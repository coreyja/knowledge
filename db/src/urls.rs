use readability;
use readability::extractor;

use reqwest;
pub use sqlx;
use sqlx::types::chrono;
pub use sqlx::PgPool;
use url::Url;
use uuid::Uuid;


use crate::openai_utils::{generate_categories, generate_summary};

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

#[derive(sqlx::FromRow, Debug)]
pub struct Markdown {
    pub markdown_id: Uuid,
    pub title: Option<String>,
    pub content_md: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Category {
    pub markdown_id: Uuid,
    pub category: Option<String>,
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
    let response = reqwest::get(url).await?;
    let html = response.text().await?;
    Ok(html)
}

fn clean_raw_html(raw_html: &str, url: &Url) -> color_eyre::Result<String> {
    let mut raw_html_cursor = std::io::Cursor::new(raw_html);
    let article = extractor::extract(&mut raw_html_cursor, url)?;
    Ok(article.content)
}

async fn store_category(
    pool: &PgPool,
    markdown_id: Uuid,
    category: &str,
) -> color_eyre::Result<Category> {
    let result = sqlx::query_as!(
        Category,
        "INSERT INTO Category (markdown_id, category) VALUES ($1, $2) RETURNING *",
        markdown_id,
        category.to_string()
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

async fn store_markdown(
    pool: &PgPool,
    page_snapshot_id: Uuid,
    cleaned_html: &str,
) -> color_eyre::Result<Markdown> {
    let markdown_content = html2md::parse_html(cleaned_html);
    let summary = generate_summary(&markdown_content).await?;
    println!("Summary: {summary}");

    let markdown_result = sqlx::query_as!(
        Markdown,
        "INSERT INTO Markdown (markdown_id, content_md) VALUES ($1, $2) RETURNING *",
        page_snapshot_id,
        markdown_content
    )
    .fetch_one(pool)
    .await?;

    let category = generate_categories(&markdown_content).await?;
    let category_result = store_category(pool, markdown_result.markdown_id, &category).await?;
    println!("Category result: {category_result:?}");

    Ok(markdown_result)
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

    let markdown_result = store_markdown(pool, result.page_snapshot_id, &cleaned_html).await?;
    println!("Markdown result: {markdown_result:?}");

    Ok(result)
}

pub async fn process_page_snapshot(pool: &PgPool, page: Page) -> color_eyre::Result<()> {
    let outcome = store_raw_html_in_page_snapshot(pool, page).await?;
    println!("Outcome: {outcome:?}");
    Ok(())
}

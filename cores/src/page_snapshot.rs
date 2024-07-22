use color_eyre::eyre::{eyre, Result};
use readability::extractor;
use reqwest;
use scraper::{Html, Selector};
use sqlx::types::chrono;
use url::Url;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct PageSnapShot {
    pub page_snapshot_id: Uuid,
    pub page_id: Uuid,
    pub raw_html: String,
    pub cleaned_html: String,
    pub fetched_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn download_raw_html(url: &str) -> color_eyre::Result<String> {
    let response = reqwest::get(url).await?;
    let html = response.text().await?;
    Ok(html)
}

pub fn clean_raw_html(raw_html: &str, url: &Url) -> color_eyre::Result<String> {
    let mut raw_html_cursor = std::io::Cursor::new(raw_html);
    let article = extractor::extract(&mut raw_html_cursor, url)?;
    Ok(article.content)
}

pub fn extract_title(content: &str) -> Result<Option<String>> {
    let document = Html::parse_document(content);
    let title_selector =
        Selector::parse("title").map_err(|_| eyre!("Could not make title selector"))?;

    Ok(document
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect()))
}

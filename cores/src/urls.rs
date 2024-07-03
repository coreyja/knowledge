pub use sqlx;
pub use sqlx::PgPool;
use uuid::Uuid;

use crate::openai_utils::generate_summary;

#[derive(sqlx::FromRow, Debug, Clone)]
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

#[allow(clippy::must_use_candidate)]
impl AddUrlOutcome {
    pub fn page(&self) -> &Page {
        match self {
            AddUrlOutcome::Created(page) | AddUrlOutcome::Existing(page) => page,
        }
    }
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

pub async fn generate_and_store_summary(
    pool: &PgPool,
    markdown_id: Uuid,
    cleaned_html: &str,
) -> color_eyre::Result<()> {
    let summary = generate_summary(cleaned_html).await?;

    sqlx::query!(
        "UPDATE Markdown SET summary = $1 WHERE markdown_id = $2",
        summary,
        markdown_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// pub async fn persist_article(pool: &PgPool, page: Page) -> color_eyre::Result<PageSnapShot> {
//     let raw_html = download_raw_html(&page.url).await?;
//     // let current_time = chrono::Utc::now();
//     let url = Url::parse(&page.url)?;
//     let cleaned_html = clean_raw_html(&raw_html, &url)?;

//     let markdown_result = store_in_markdown_table(pool, result.page_snapshot_id, &cleaned_html).await?;
//     generate_and_store_summary(
//         pool,
//         markdown_result.markdown_id,
//         &markdown_result.content_md,
//     )
//     .await?;

//     Ok(result)
// }

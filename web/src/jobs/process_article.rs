use crate::AppState;
use cja::{app_state::AppState as _, jobs::Job};

use url::Url;
use uuid::Uuid;

use cores::markdown::store_in_markdown_table;
use cores::page_snapshot::{clean_raw_html, download_raw_html, PageSnapShot};
use cores::urls::{generate_and_store_summary, Page};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ProcessArticle {
    pub page_id: Uuid,
}

#[async_trait::async_trait]
impl Job<AppState> for ProcessArticle {
    const NAME: &'static str = "process_article::ProcessArticle";

    async fn run(&self, app_state: AppState) -> cja::Result<()> {
        let db = app_state.db();

        let page = sqlx::query_as!(Page, "SELECT * FROM pages WHERE page_id = $1", self.page_id)
            .fetch_one(db)
            .await?;

        let raw_html = download_raw_html(&page.url).await?;
        let url_parsed = Url::parse(&page.url)?;
        let current_time = chrono::Utc::now();
        let cleaned_html = clean_raw_html(&raw_html, &url_parsed)?;

        let page_snapshot = sqlx::query_as!(
            PageSnapShot,
            "INSERT INTO page_snapshots (raw_html, fetched_at, cleaned_html, page_id) 
        VALUES ($1, $2, $3, $4) RETURNING *",
            raw_html,
            current_time,
            cleaned_html,
            page.page_id
        )
        .fetch_one(db)
        .await?;

        let markdown = store_in_markdown_table(db, page_snapshot).await?;

        generate_and_store_summary(db, markdown.markdown_id, &cleaned_html).await?;

        Ok(())
    }
}

use crate::AppState;
use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;

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

    async fn run(&self, app_state: AppState) -> miette::Result<()> {
        let db = app_state.db();

        let page = sqlx::query_as!(Page, "SELECT * FROM pages WHERE page_id = $1", self.page_id)
            .fetch_one(db)
            .await
            .into_diagnostic()?;

        let raw_html = download_raw_html(&page.url).await.unwrap();
        let url_parsed = Url::parse(&page.url).unwrap();
        let current_time = chrono::Utc::now();
        let cleaned_html = clean_raw_html(&raw_html, &url_parsed).unwrap();

        let page_snapshot = sqlx::query_as!(
            PageSnapShot,
            "INSERT INTO PageSnapShot (raw_html, fetched_at, cleaned_html, page_id) 
        VALUES ($1, $2, $3, $4) RETURNING *",
            raw_html,
            current_time,
            cleaned_html,
            page.page_id
        )
        .fetch_one(db)
        .await
        .unwrap(); // Insert a new page snapshot into the database and return the inserted record.

        let markdown = store_in_markdown_table(db, page_snapshot).await.unwrap();
        let markdown_id = markdown.markdown_id;

        generate_and_store_summary(db, markdown_id, &cleaned_html)
            .await
            .unwrap();

        Ok(())
    }
}

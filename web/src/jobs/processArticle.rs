use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;

use crate::AppState;

use db::openai_utils::{generate_summary, generate_categories};
use db::urls::{store_markdown, store_category, raw_html, clean_raw_html, Page, PageSnapshot};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Cleanup;



#[async_trait::async_trait]
impl Job<AppState> for Cleanup {
    const NAME: &'static str = "sessions::processArticle";

    async fn run(&self, app_state: AppState) -> miette::Result<()> {
        let db = app_state.db();

        let url = sqlx::query_as!(
            Page,
            "SELECT page_id, url FROM pages WHERE page_id = $1",
            page_id,
            url
        )
        .fetch_all(db)
        .await
        .into_diagnostic()?;

        // store the html content in the page snapshot
        store_html_content_in_page_snapshot(db, url).await;


        // get the page snapshot 
        let page_snapshot = sqlx::query_as!(
            PageSnapshot,
            "SELECT * FROM page_snapshots WHERE page_id = $1",
            page_id
        )
        .fetch_all(db)
        .await
        .into_diagnostic()?;

        let raw_html = raw_html(page_snapshot.raw_html).await;
        let cleaned_html = clean_raw_html(raw_html, url.url).await;

        store_markdown(db, page_snapshot.page_snapshot_id, &cleaned_html).await;

        let summary = generate_summary(cleaned_html).await;
        let categories = generate_categories(cleaned_html).await;

        store_category(db, categories, page_snapshot.page_snapshot_id).await;

        Ok(())
    }
}

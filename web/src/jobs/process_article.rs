use crate::AppState;
use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;
use url::Url;
use uuid::Uuid;

use db::openai_utils::{generate_categories, generate_summary};
use db::urls::{
    clean_raw_html, download_raw_html, store_category, store_html_content_in_page_snapshot,
    store_markdown, Page, PageSnapShot,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ProcessArticle {
    pub page_id: Uuid,
}

#[async_trait::async_trait]
impl Job<AppState> for ProcessArticle {
    const NAME: &'static str = "process_article::processArticle";

    async fn run(&self, app_state: AppState) -> miette::Result<()> {
        let db = app_state.db();

        let page = sqlx::query_as!(Page, "SELECT * FROM pages WHERE page_id = $1", self.page_id)
            .fetch_one(db)
            .await
            .into_diagnostic()?;

        // store the html content in the page snapshot
        store_html_content_in_page_snapshot(db, page.clone()).await;

        // get the page snapshot
        let page_snapshot = sqlx::query_as!(
            PageSnapShot,
            "SELECT * FROM PageSnapshot WHERE page_id = $1",
            self.page_id
        )
        .fetch_one(db)
        .await
        .into_diagnostic()?;

        let raw_html = download_raw_html(&page.url).await.unwrap();

        let url = Url::parse(&page.url).into_diagnostic()?;
        let cleaned_html = clean_raw_html(&raw_html, &url).unwrap();

        store_markdown(db, page_snapshot.page_snapshot_id, &cleaned_html).await;

        let summary = generate_summary(&cleaned_html).await.unwrap();
        let category = generate_categories(&cleaned_html).await.unwrap();

        store_category(db, page_snapshot.page_snapshot_id, &category).await;
        
        Ok(())
    }
}

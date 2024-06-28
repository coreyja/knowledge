use crate::AppState;
use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;

use uuid::Uuid;

use db::urls::{store_html_content_in_page_snapshot, Page};

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
        let page_snapshot = store_html_content_in_page_snapshot(db, page.clone()).await;

        println!("{:?}", page_snapshot);

        Ok(())
    }
}

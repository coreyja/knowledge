use crate::AppState;
use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;

use uuid::Uuid;

use db::urls::{persist_article, Markdown, Page};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ProcessArticle {
    pub page_id: Uuid,
    pub markdown_id: Uuid,
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

        let markdown = sqlx::query_as!(
            Markdown,
            "SELECT * FROM markdown WHERE markdown_id = $1",
            self.markdown_id
        )
        .fetch_one(db)
        .await
        .into_diagnostic()?;

        persist_article(db, page, markdown.clone()).await.unwrap();

        Ok(())
    }
}

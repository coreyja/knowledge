use cja::{app_state::AppState as _, jobs::Job};
use miette::IntoDiagnostic;

use crate::AppState;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Cleanup;

#[async_trait::async_trait]
impl Job<AppState> for Cleanup {
    const NAME: &'static str = "sessions::cleanup";

    async fn run(&self, app_state: AppState) -> miette::Result<()> {
        let db = app_state.db();

        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < now() - interval '1 day'
            "#,
        )
        .execute(db)
        .await
        .into_diagnostic()?;

        Ok(())
    }
}

use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use cja::jobs::Job;
use tracing::info;

use crate::{jobs::process_article::ProcessArticle, AppState, WebResult};

use cores::{urls::add_url, users::User};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleForm {
    url: String,
}

#[axum::debug_handler(state = AppState)]
pub async fn insert_article_handler(
    State(state): State<AppState>,
    user: User,
    Form(form): Form<ArticleForm>,
) -> WebResult<impl IntoResponse> {
    info!("Received request to insert article: {}", form.url); // Log the received URL

    let url = form.url;
    let user_id = user.user_id;

    let page = add_url(&state.db, &url, user_id, &true).await?;
    let page_id: uuid::Uuid = page.page().page_id;

    let process_article = ProcessArticle { page_id };
    process_article
        .enqueue(state.clone(), "insert_article_handler".to_string())
        .await?;

    Ok(Redirect::to(&format!(
        "/articles/{page_id}?flash[success]=Article added"
    )))
}

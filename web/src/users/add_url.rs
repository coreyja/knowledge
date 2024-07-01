use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use cja::jobs::Job;

use crate::{jobs::process_article::ProcessArticle, AppState, WebResult};

use db::{urls::add_url, users::User};
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
    let url = form.url;
    let user_id = user.user_id;

    let page = add_url(&state.db, &url, user_id, &true).await?;
    let page_id = page.page().page_id;
    let process_article = ProcessArticle { page_id };
    process_article
        .enqueue(state.clone(), "insert_article_handler".to_string())
        .await?;

    Ok(Redirect::to(&format!(
        "/article/{page_id}?flash[success]=Article added"
    )))
}

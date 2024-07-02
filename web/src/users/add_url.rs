use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use cja::jobs::Job;
use tracing::info;
use url::Url;

use crate::{jobs::process_article::ProcessArticle, AppState, WebResult};

use db::{
    urls::{
        add_url, clean_raw_html, download_raw_html, generate_and_store_summary, store_markdown,
    },
    users::User,
};
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
    info!("Received request to insert article: {}", form.url);

    let url = form.url;
    let user_id = user.user_id;

    let page = add_url(&state.db, &url, user_id, &true).await?;
    let page_id = page.page().page_id;

    let raw_html = download_raw_html(&url).await?;
    let url_parsed = Url::parse(&url)?;
    let cleaned_html = clean_raw_html(&raw_html, &url_parsed)?;

    let markdown = store_markdown(&state.db, page_id, &cleaned_html)
        .await
        .unwrap();
    let markdown_id = markdown.markdown_id;

    generate_and_store_summary(&state.db, markdown_id, &cleaned_html)
        .await
        .unwrap();

    let process_article = ProcessArticle {
        page_id,
        markdown_id,
    };
    process_article
        .enqueue(state.clone(), "insert_article_handler".to_string())
        .await?;

    info!("Successfully processed article: {}", markdown_id);

    Ok(Redirect::to(&format!(
        "/articles/{markdown_id}?flash[success]=Article added"
    )))
}

use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};

use db::{urls::add_url, users::User};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct ArticleForm {
    url: String,
}

#[axum::debug_handler(state = AppState)]
pub async fn insert_article_handler(
    State(state): State<AppState>,
    user: User,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    let url = form.url;
    let user_id = user.user_id;

    tracing::info!("{}", url);
    match add_url(&state.db, &url, user_id, &true).await {
        Ok(_) => Redirect::to("/dashboard?flash[success]=Article added"),
        Err(e) => {
            eprintln!("Error adding URL: {e:?}");
            Redirect::to("/dashboard?flash[error]=Could not add article")
        }
    }
}

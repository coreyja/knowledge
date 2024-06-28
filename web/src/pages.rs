use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use db::{
    urls::{Page, PageSnapShot},
    users::User,
};

use crate::{
    templates::{Template, TemplatedPage},
    AppState, WebResult,
};

use uuid::Uuid;

pub async fn home(t: Template, user: Option<User>) -> Response {
    match user {
        Some(user) => user_dashboard(t, user).await.into_response(),
        None => landing(t).await.into_response(),
    }
}

pub async fn landing(t: Template) -> TemplatedPage {
    t.render(maud::html! {
        h1 { "Knowledge" }
        h2 { "A cool app that needs a new name" }

        p { "Welcome to Knowledge! Please log in." }
        a href="/login" { "Login" }
    })
}

pub async fn user_dashboard(t: Template, user: User) -> TemplatedPage {
    t.render(maud::html! {
        h1 { "Dashboard" }
        p { "Welcome, " (user.user_name) }

        a href="/logout" { "Logout" }

        h3 {  "Insert Article" }
        form method="post" action="/insert_articles" {
            label for="url" { "URL :" }
            input id="url" name="url" placeholder="Insert URL" {}
            br;
            input type="submit" value="Submit";
        }
    })
}

#[axum::debug_handler(state = AppState)]
pub async fn article_detail(
    t: Template,
    Path(article_id): Path<Uuid>,
    State(state): State<AppState>,
) -> WebResult<Response> {
    let article_id = sqlx::query_as!(Page, "SELECT * FROM pages WHERE page_id = $1", article_id)
        .fetch_one(&state.db)
        .await?;

    let page_snapshot = sqlx::query_as!(
        PageSnapShot,
        "SELECT * FROM PageSnapshot WHERE page_id = $1",
        article_id.page_id
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(t.render(maud::html! {
        @if let Some(page_snapshot) = page_snapshot {
            p { (page_snapshot.summary) }
        } @else {
            p { "No page snapshot found" }
        }
    })
    .into_response())
}

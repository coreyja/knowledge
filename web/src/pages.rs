use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use cores::{markdown::Markdown, page_snapshot::PageSnapShot, urls::Page, users::User};

use crate::{
    templates::{Template, TemplatedPage},
    AppState, WebResult,
};

use tracing::info;
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
        form method="post" action="/articles" {
            label for="url" { "URL :" }
            input id="url" name="url" placeholder="Insert URL" {}
            br;
            input type="submit" value="Submit";
        }

        h3 {"My Articles"}
        p {
            @let url = format!("/my_articles/{}", user.user_id);
            a href=(url) { "click" }
        }


    })
}

#[axum::debug_handler(state = AppState)]
pub async fn article_detail(
    t: Template,
    Path(page_id): Path<Uuid>,
    State(state): State<AppState>,
) -> WebResult<Response> {
    info!("Fetching article_ID: {}", page_id);

    let article = sqlx::query_as!(Page, "SELECT * FROM pages WHERE page_id = $1", page_id)
        .fetch_one(&state.db)
        .await?;

    let page_snapshot = sqlx::query_as!(
        PageSnapShot,
        "SELECT * FROM pagesnapshot WHERE page_id = $1 ORDER BY fetched_at DESC LIMIT 1",
        article.page_id
    )
    .fetch_optional(&state.db)
    .await?;

    let markdown = if let Some(page_snapshot) = page_snapshot {
        sqlx::query_as!(
            Markdown,
            "SELECT * FROM markdown WHERE page_snapshot_id = $1",
            page_snapshot.page_snapshot_id
        )
        .fetch_optional(&state.db)
        .await?
    } else {
        None
    };
    info!("Fetched Article: {:?}", article);

    info!("Fetched markdown MD: {:?}", markdown);

    Ok(t.render(maud::html! {
        @if let Some(markdown) = markdown {
            p { (markdown.summary) }
        } @else {
            p { "Generating snapshot....." }
        }
    })
    .into_response())
}

#[axum::debug_handler(state = AppState)]
pub async fn my_articles(
    t: Template,
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
) -> WebResult<Response> {
    info!("Received request for user_id: {}", user_id);
    let my_articles = sqlx::query_as!(Page, "SELECT * FROM pages WHERE user_id = $1", user_id)
        .fetch_all(&state.db)
        .await?;

    Ok(t.render(maud::html! {
        h1 { "My Articles" }
        table {
            tr {
                th { "URL" }
                th { "Actions" }
            }
            @for article in my_articles {
                @let article_url = format!("/articles/{}", article.page_id);
                tr {
                    td { (article.url) }
                    td { a href=(article_url) { "View" } }
                }
            }
        }
    })
    .into_response())
}

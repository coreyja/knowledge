use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use cores::{
    category::Category, markdown::Markdown, page_snapshot::PageSnapShot, urls::Page, users::User,
};

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

        h3 {
            a href="/articles" { "My Articles" }
        }

        h3 {
            a href="/categories" { "My Categories" }
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
        "SELECT * FROM page_snapshots WHERE page_id = $1 ORDER BY fetched_at DESC LIMIT 1",
        article.page_id
    )
    .fetch_optional(&state.db)
    .await?;

    let stuff = if let Some(page_snapshot) = page_snapshot {
        let markdown = sqlx::query_as!(
            Markdown,
            "SELECT * FROM markdowns WHERE page_snapshot_id = $1",
            page_snapshot.page_snapshot_id
        )
        .fetch_optional(&state.db)
        .await?;

        if let Some(markdown) = markdown {
            let categories = sqlx::query_as!(
                Category,
                "SELECT categories.*
                FROM categories
                JOIN markdown_categories USING (category_id)
                WHERE markdown_id = $1",
                markdown.markdown_id
            )
            .fetch_all(&state.db)
            .await?;
            Some((page_snapshot, markdown, categories))
        } else {
            None
        }
    } else {
        None
    };
    let inner_html = if let Some((page_snapshot, markdown, categories)) = stuff {
        maud::html! {
            p { b { "Fetched At: " } (page_snapshot.fetched_at) }
            ul {
                @for category in categories {
                    li { b { "Category: " } (category.category) }
                }
            }
            @if let Some(title) = markdown.title {
                h1 { b { "Title: " } (title) }
            }
            @if let Some(summary) = markdown.summary {
                p { b { "Summary: " } (summary) }
            } @else {
                p data-controller="loader" { "Generating summary....." }
            }
        }
    } else {
        maud::html! {
            p data-controller="loader" { "Generating snapshot....." }
        }
    };

    Ok(t.render(maud::html! {
        h1 { "Article Detail" }
        h2 { "URL: " (article.url) }
        div { (inner_html) }
    })
    .into_response())
}

pub async fn my_articles(
    t: Template,
    State(state): State<AppState>,
    user: User,
) -> WebResult<Response> {
    info!("Received request for user_id: {}", user.user_id);
    let my_articles = sqlx::query_as!(Page, "SELECT * FROM pages WHERE user_id = $1", user.user_id)
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
                    td {
                        a href=(article_url) { "View" }
                    }
                }
            }
        }
    })
    .into_response())
}

#[axum::debug_handler(state = AppState)]
pub async fn my_categories(
    t: Template,
    State(state): State<AppState>,
    user: User,
) -> WebResult<Response> {
    info!("Fetching categories for user_id: {}", user.user_id);

    let categories = sqlx::query!(
        "SELECT DISTINCT c.* 
         FROM categories c
         JOIN markdown_categories cm USING (category_id)
         JOIN markdowns m USING (markdown_id)
         JOIN page_snapshots ps USING (page_snapshot_id)
         JOIN pages p USING (page_id)
         WHERE p.user_id = $1",
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(t.render(maud::html! {
        h1 { "My Categories" }
        ul {
            @for category in categories {
                @let category_name = category.category;
                li {
                    a href=(format!("/categories/{}", category.category_id)) { (category_name) }
                }
            }
        }
    })
    .into_response())
}

#[axum::debug_handler(state = AppState)]
pub async fn articles_by_category(
    t: Template,
    Path(category_id): Path<Uuid>,
    State(state): State<AppState>,
    user: User,
) -> WebResult<Response> {
    info!("Fetching articles for category_id: {}", category_id);

    let articles = sqlx::query!(
        r#"
        SELECT m.summary, p.url, p.page_id, c.category, m.title
        FROM categories c
        JOIN markdown_categories cm USING (category_id)
        JOIN markdowns m USING (markdown_id)
        JOIN page_snapshots ps USING (page_snapshot_id)
        JOIN pages p USING (page_id)
        WHERE c.category_id = $1 AND p.user_id = $2
        "#,
        category_id,
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(t.render(maud::html! {
        h1 { "Articles in Category" }
        ul {
            @for article in articles {
                li {
                    a href=(format!("/articles/{}", article.page_id)) { (article.url) }
                    @if let Some(title) = article.title {
                        p { b { "Title: " } (title) }
                    }
                    p { b { "Category: " } (article.category) }
                    @if let Some(summary) = article.summary {
                        p { b { "Summary: " } (summary) }
                    }
                }
            }
        }
    })
    .into_response())
}

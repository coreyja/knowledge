use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use cores::{
    markdown::{self, Markdown},
    page_snapshot::PageSnapShot,
    urls::Page,
    users::User,
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
        "SELECT * FROM pagesnapshot WHERE page_id = $1 ORDER BY fetched_at DESC LIMIT 1",
        article.page_id
    )
    .fetch_optional(&state.db)
    .await?;

    let markdown = if let Some(page_snapshot) = page_snapshot {
        let markdown = sqlx::query_as!(
            Markdown,
            "SELECT * FROM markdown WHERE page_snapshot_id = $1",
            page_snapshot.page_snapshot_id
        )
        .fetch_optional(&state.db)
        .await?;

        if let Some(markdown) = markdown {
            let categories = sqlx::query_as!(
                Category,
                "SELECT * FROM category WHERE markdown_id = $1",
                markdown.markdown_id
            )
            .fetch_all(&state.db)
            .await?;
            Some((markdown, categories))
        } else {
            None
        }
    } else {
        None
    };

    let rendered_html = if let Some((markdown, categories)) = markdown {
        maud::html! {
            ul {
                @for category in categories {
                    li { b { "Category: " } (category.category.unwrap_or("No category".to_string())) }
                }
            }
            p { b { "Summary: " }(markdown.summary.as_str()) }
        }
    } else {
        maud::html! {
            p { "Generating snapshot....." }
        }
    };

    Ok(t.render(rendered_html).into_response())
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
         FROM category c
         JOIN categorymarkdown cm ON c.category_id = cm.category_id
         JOIN markdown m ON cm.markdown_id = m.markdown_id
         JOIN pagesnapshot ps ON m.page_snapshot_id = ps.page_snapshot_id
         JOIN pages p ON ps.page_id = p.page_id
         WHERE p.user_id = $1",
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(t.render(maud::html! {
        h1 { "My Categories" }
        ul {
            @for category in categories {
                @let category_name = category.category.unwrap_or("No category".to_string());
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
        SELECT m.summary, p.url, p.page_id, c.category
        FROM category c
        JOIN categorymarkdown cm ON c.category_id = cm.category_id
        JOIN markdown m ON cm.markdown_id = m.markdown_id
        JOIN pagesnapshot ps ON m.page_snapshot_id = ps.page_snapshot_id
        JOIN pages p ON ps.page_id = p.page_id
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
                    a href=(format!("/articles/{}", article.page_id))
                    p { b { "URL: " } (article.url) }
                    p { b { "Category: " } (article.category.as_deref().unwrap_or("No category")) }
                    p { b { "Summary: " } (article.summary) }
                }
            }
        }
    })
    .into_response())
}

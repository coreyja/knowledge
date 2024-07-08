use axum::routing::get;
use axum::routing::post;

use crate::pages::articles_by_category;
use crate::pages::{article_detail, my_articles, my_categories};

use crate::{
    admin,
    pages::{home, landing, user_dashboard},
    users, AppState,
};

pub fn routes(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/", get(home))
        .nest("/_", admin::routes())
        .route("/hello", get(landing))
        .route("/dashboard", get(user_dashboard))
        .route("/login", get(users::login::get).post(users::login::post))
        .route("/signup", get(users::signup::get).post(users::signup::post))
        .route("/logout", get(users::login::logout))
        .route(
            "/articles",
            post(users::add_url::insert_article_handler).get(my_articles),
        )
        .route("/articles/:article_id", get(article_detail))
        .route("/categories", get(my_categories))
        .route("/categories/:category_id", get(articles_by_category))
        .with_state(app_state)
}

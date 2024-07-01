use axum::routing::get;
use axum::routing::post;

use crate::pages::article_detail;
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
            "/insert_articles",
            post(users::add_url::insert_article_handler),
        )
        .route("/article/:article_id", get(article_detail))
        .with_state(app_state)
}

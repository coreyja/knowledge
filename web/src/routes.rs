use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::routing::post;
use axum::Form;
use axum::Json;
use cja::app_state::AppState as _;
use cores::users::User;
use serde_json::Value;

use crate::pages::article_detail;
use crate::pages::my_articles;
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
        .route("/slack/command", post(slack_command))
        .route("/slack/login", get(slack_login))
        .with_state(app_state)
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct SlackCommandRequest {
    api_app_id: String,
    channel_id: String,
    channel_name: String,
    command: String,
    response_url: String,
    text: String,
    token: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
}

#[axum::debug_handler]
pub async fn slack_command(
    State(state): State<AppState>,
    Form(json): Form<SlackCommandRequest>,
) -> Json<Value> {
    dbg!("SlackCommand");
    dbg!(&json);

    match json.command.as_str() {
        "/login" => do_login_command(state, json).await.unwrap(),
        "/article" => do_article_command(state, json).await.unwrap(),
        _ => Json(serde_json::json!({
            "response_type": "in_channel",
            "text": "Unknown command"
        })),
    }
}

#[allow(clippy::unused_async, unused_variables)]
pub async fn do_login_command(
    state: AppState,
    json: SlackCommandRequest,
) -> color_eyre::Result<Json<Value>> {
    let slack_user_link = sqlx::query!(
        "
    INSERT INTO SlackUserLink (slack_user_id, slack_username) values ($1, $2) RETURNING *",
        json.user_id,
        json.user_name
    )
    .fetch_one(state.db())
    .await?;

    let base_url = "https://guiding-raptor-infinitely.ngrok-free.app";
    let url = format!(
        "{base_url}/slack/login?slack_user_link={}",
        slack_user_link.slack_user_link_id
    );

    Ok(Json(serde_json::json!({
        "response_type": "in_channel",
        "text": format!("To Login please visit: {url}"),
    })))
}

#[allow(clippy::unused_async, unused_variables)]
pub async fn do_article_command(
    state: AppState,
    json: SlackCommandRequest,
) -> color_eyre::Result<Json<Value>> {
    Ok(Json(serde_json::json!({
        "response_type": "in_channel",
        "text": "Article command"
    })))
}

#[derive(serde::Deserialize, Debug)]
pub struct SlackLoginQuery {
    slack_user_link: String,
}

pub async fn slack_login(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<SlackLoginQuery>,
) -> &'static str {
    let slack_user_link = sqlx::query!(
        "SELECT * FROM SlackUserLink WHERE slack_user_link_id = $1",
        uuid::Uuid::parse_str(&query.slack_user_link).unwrap()
    )
    .fetch_one(state.db())
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO SlackUserAssociation (slack_user_link_id, user_id, slack_user_id, slack_username) VALUES ($1, $2, $3, $4) RETURNING *",
        slack_user_link.slack_user_link_id,
        user.user_id,
        slack_user_link.slack_user_id,
        slack_user_link.slack_username
    ).fetch_one(state.db()).await.unwrap();

    "Login Worked"
}

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use cja::app_state::AppState as _;
use db::users::User;
use miette::IntoDiagnostic;
use password_auth::generate_hash;

use crate::{sessions::Session, templates::Template, AppState, WebResult};

pub async fn get(t: Template) -> impl IntoResponse {
    t.render(maud::html! {
      form method="post" action="/signup" {
        input type="text" name="username" placeholder="Username";
        input type="password" name="password" placeholder="Password";
        input type="submit" value="Signup";
      }
    })
}

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}

pub async fn post(
    session: Session,
    State(state): State<AppState>,
    form_data: Form<FormData>,
) -> WebResult<Response> {
    let existing_user = sqlx::query_as!(
        User,
        "SELECT * FROM Users WHERE user_name = $1",
        form_data.username
    )
    .fetch_optional(state.db())
    .await?;

    if existing_user.is_some() {
        return Ok(
            Redirect::to("/signup?flash[error]=Sorry this username is taken.").into_response(),
        );
    }

    let password_for_hash = form_data.password.clone();
    let password_hash =
        tokio::task::spawn_blocking(move || generate_hash(password_for_hash)).await?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO Users (user_name, password_hash) VALUES ($1, $2) RETURNING *",
        form_data.username,
        password_hash
    )
    .fetch_one(state.db())
    .await?;

    sqlx::query!(
        "UPDATE Sessions SET user_id = $1, updated_at = now() WHERE session_id = $2",
        user.user_id,
        session.session_id
    )
    .execute(state.db())
    .await?;

    Ok(axum::response::Redirect::to("/dashboard").into_response())
}

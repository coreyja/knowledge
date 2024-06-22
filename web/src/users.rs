use axum::{
    extract::{FromRequestParts, State},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use cja::app_state::AppState as _;

use thiserror::Error;

pub use db::users::User;

use crate::{
    sessions::{Session, SessionError},
    AppState,
};

#[derive(Error, Debug)]
pub enum ExtractUserError {
    #[error("No session")]
    NoSession,
    #[error("No user id")]
    NoUserId,
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for ExtractUserError {
    fn into_response(self) -> Response {
        match self {
            Self::NoSession | Self::NoUserId => {
                axum::response::Redirect::temporary("/login").into_response()
            }
            Self::SqlxError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl From<SessionError> for ExtractUserError {
    fn from(_: SessionError) -> Self {
        ExtractUserError::NoSession
    }
}

#[async_trait::async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = ExtractUserError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, ExtractUserError> {
        let session = Session::from_request_parts(parts, state).await?;

        let user_id = session.user_id.ok_or(ExtractUserError::NoUserId)?;

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM Users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(state.db())
        .await?;

        Ok(user)
    }
}

pub async fn login_get() -> impl IntoResponse {
    maud::html! {
        form method="post" action="/login" {
            input type="text" name="username" placeholder="Username";
            input type="password" name="password" placeholder="Password";
            input type="submit" value="Login";
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginFromData {
    username: String,
    password: String,
}

// #[axum::debug_handler(state = AppState)]
pub async fn login_post(
    session: Session,
    State(state): State<AppState>,
    form: Form<LoginFromData>,
) -> Result<Redirect, Redirect> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM Users
        WHERE user_name = $1
        "#,
        form.username
    )
    .fetch_optional(state.db())
    .await
    .map_err(|_| Redirect::to("/login"))?
    .ok_or_else(|| Redirect::to("/login"))?;

    sqlx::query!(
        "UPDATE Sessions SET user_id = $1 WHERE session_id = $2",
        user.user_id,
        session.session_id
    )
    .execute(state.db())
    .await
    .map_err(|_| Redirect::to("/login"))?;

    Ok(Redirect::to("/dashboard"))
}

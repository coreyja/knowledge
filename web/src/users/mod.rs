use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use cja::app_state::AppState as _;

use thiserror::Error;

pub use cores::users::User;

use crate::{
    sessions::{Session, SessionError},
    AppState,
};

pub mod add_url;
pub mod login;
pub mod signup;

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

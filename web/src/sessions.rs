use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{self},
    response::{IntoResponse, Response},
};
use cja::{app_state::AppState as _, tower_cookies::Cookies};

use uuid::Uuid;

use crate::AppState;

#[derive(Debug)]
pub struct Session {
    #[allow(clippy::struct_field_names)]
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Session {
    async fn create(state: &AppState, cookies: &Cookies) -> Result<Self, SessionError> {
        let session_id = Uuid::new_v4();
        let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

        let session = sqlx::query_as!(
            Session,
            r"
      INSERT INTO Sessions (session_id, expires_at)
      VALUES ($1, $2)
      RETURNING *
      ",
            session_id,
            expires_at
        )
        .fetch_one(state.db())
        .await
        .map_err(|_| SessionError)?;

        let private = cookies.private(state.cookie_key());

        let session_cookie =
            cja::tower_cookies::Cookie::build(("session_id", session.session_id.to_string()))
                .path("/")
                .http_only(true)
                .secure(true)
                .expires(None);
        private.add(session_cookie.into());

        Ok(session)
    }
}

pub struct SessionError;

impl IntoResponse for SessionError {
    fn into_response(self) -> Response<Body> {
        http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = SessionError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Ok(cookies) = Cookies::from_request_parts(parts, state).await else {
            return Err(SessionError);
        };

        let private = cookies.private(state.cookie_key());

        let session_cookie = private.get("session_id");

        let Some(session_cookie) = session_cookie else {
            return Session::create(state, &cookies).await;
        };

        let session_id = session_cookie.value().to_string();
        let Ok(session_id) = uuid::Uuid::parse_str(&session_id) else {
            return Session::create(state, &cookies).await;
        };

        let Ok(session) = sqlx::query_as!(
            Session,
            r"
        SELECT *
        FROM Sessions
        WHERE session_id = $1
        ",
            session_id
        )
        .fetch_optional(state.db())
        .await
        else {
            return Session::create(state, &cookies).await;
        };

        if let Some(session) = session {
            Ok(session)
        } else {
            Session::create(state, &cookies).await
        }
    }
}

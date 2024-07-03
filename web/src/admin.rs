use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::get,
};
use cja::app_state::AppState as _;
use color_eyre::eyre::Context;
use cores::users::User;

use crate::{err, templates::Template, users::ExtractUserError, AppState, WebResult};

async fn simple_error(_: AdminUser) -> WebResult<()> {
    Err(color_eyre::eyre::eyre!("This is a test error"))?
}

async fn sql_error(_: AdminUser, State(app_state): State<AppState>) -> WebResult<()> {
    let _ = sqlx::query("This is not valid sql")
        .fetch_one(app_state.db())
        .await
        .wrap_err("We meant for this to fail but lets see the error chain")?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum ExtractAdminError {
    #[error(transparent)]
    ExtractUserError(ExtractUserError),
    #[error("User is not an admin")]
    NotAdmin,
}

impl IntoResponse for ExtractAdminError {
    fn into_response(self) -> axum::response::Response {
        // We only want to report a SQL error. Others we can return a 401
        if let ExtractAdminError::ExtractUserError(ExtractUserError::SqlxError(e)) = self {
            err::Error::from(e).into_response();
        };

        StatusCode::UNAUTHORIZED.into_response()
    }
}

struct AdminUser(User);

#[async_trait::async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ExtractAdminError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state)
            .await
            .map_err(ExtractAdminError::ExtractUserError)?;

        if is_admin(&user) {
            Ok(AdminUser(user))
        } else {
            Err(ExtractAdminError::NotAdmin)
        }
    }
}

fn is_admin(user: &User) -> bool {
    let admin_usernames = std::env::var("ADMIN_USERNAMES").unwrap_or_else(|_| String::new());
    let admin_usernames = admin_usernames
        .split(',')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    admin_usernames.contains(&user.user_name.as_str())
}

async fn hello(t: Template, admin: AdminUser) -> WebResult<impl IntoResponse> {
    Ok(t.render(maud::html! {
      h1 { "Hello Admin "  (admin.0.user_name) }

      a href="/_/test_errors/simple" { "Test Simple Error" }
      br;
      a href="/_/test_errors/sql" { "Test SQL Error" }
    }))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(hello))
        .route("/test_errors/simple", get(simple_error))
        .route("/test_errors/sql", get(sql_error))
}

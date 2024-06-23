use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use cja::{
    app_state::AppState as _,
    tower_cookies::{Cookie, Cookies},
};

use db::users::User;
use password_auth::verify_password;

use crate::{sessions::Session, templates::Template, AppState};

pub async fn get(t: Template) -> impl IntoResponse {
    t.render(maud::html! {
        form method="post" action="/login" {
            input type="text" name="username" placeholder="Username";
            input type="password" name="password" placeholder="Password";
            input type="submit" value="Login";
        }
    })
}

#[derive(serde::Deserialize)]
pub struct FromData {
    username: String,
    password: String,
}

#[axum::debug_handler(state = AppState)]
pub async fn post(
    session: Session,
    State(state): State<AppState>,
    form_data: Form<FromData>,
) -> Result<Redirect, Redirect> {
    let potential_user = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM Users
        WHERE user_name = $1
        "#,
        form_data.username
    )
    .fetch_optional(state.db())
    .await
    .map_err(|_| Redirect::to("/login"))?
    .ok_or_else(|| Redirect::to("/login"))?;

    let password_hash = potential_user
        .password_hash
        .as_ref()
        .ok_or_else(|| Redirect::to("/login"))?;

    let password_hash_to_verify = password_hash.to_string();

    let verify_password = tokio::task::spawn_blocking(move || {
        verify_password(&form_data.password, &password_hash_to_verify)
    })
    .await
    .unwrap();
    let user = match verify_password {
        Ok(()) => potential_user,
        Err(password_auth::VerifyError::PasswordInvalid) => {
            return Err(Redirect::to("/login?flash[error]=Invalid Password"));
        }
        Err(_) => {
            tracing::error!(
                user_id = potential_user.user_id.to_string(),
                "Password Hash Failed to parse",
            );
            return Err(Redirect::to(
                "/login?flash[error]=Internal Error, Please try again. Team has been notified",
            ));
        }
    };

    sqlx::query!(
        "UPDATE Sessions SET user_id = $1, updated_at = now() WHERE session_id = $2",
        user.user_id,
        session.session_id
    )
    .execute(state.db())
    .await
    .map_err(|_| Redirect::to("/login"))?;

    Ok(Redirect::to("/dashboard"))
}

pub async fn logout(session: Session, State(state): State<AppState>, cookies: Cookies) -> Redirect {
    let private = cookies.private(state.cookie_key());
    let cookie = Cookie::build("session_id").build();
    private.remove(cookie);

    sqlx::query!(
        "UPDATE Sessions SET logged_out_at = now(), updated_at = now() WHERE session_id = $1",
        session.session_id
    )
    .execute(state.db())
    .await
    .unwrap();

    Redirect::to("/")
}

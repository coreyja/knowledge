use color_eyre::Result;
use cores::{users::get_user, PgPool};

pub async fn display_users(pool: &PgPool) -> Result<()> {
    let users = get_user(pool).await?;
    if users.is_empty() {
        println!("Nothing to display");
    } else {
        for user in users {
            println!("User ID: {}, Username: {}", user.user_id, user.user_name);
        }
    }

    Ok(())
}

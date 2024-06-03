use db::PgPool;
use std::path::Path;

use crate::auth::{check_auth_status, persist_auth_session};


pub async fn sign_up(pool: &PgPool, username_opt: Option<String>) -> color_eyre::Result<()> {
    if Path::new("auth.txt").exists() {
        check_auth_status(pool).await?;
        return Ok(());
    }

    let username = if let Some(name) = username_opt {
        name
    } else {
        println!("Enter a username:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    };

    add_user(pool, &username).await?;
    println!("Signed up as: {username}");
    Ok(())
}

pub async fn add_user(pool: &PgPool, username: &str) -> color_eyre::Result<()> {
    let user_id = db::create_user(pool, username).await?;
    persist_auth_session(user_id)?;

    println!("User {username} added successfully with ID {user_id}!");
    Ok(())
}

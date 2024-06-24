use color_eyre::Result;
pub use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub password_hash: String,
}

pub async fn create_user(pool: &PgPool, user_name: &str) -> color_eyre::Result<Uuid> {
    let result = sqlx::query!(
        "INSERT INTO Users (user_name) VALUES ($1) RETURNING user_id",
        user_name
    )
    .fetch_one(pool)
    .await?;

    Ok(result.user_id)
}

pub async fn get_user(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM Users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_username_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<String>> {
    let record = sqlx::query!("SELECT user_name FROM Users WHERE user_id = $1", user_id)
        .fetch_optional(pool)
        .await?;
    Ok(record.map(|r| r.user_name))
}

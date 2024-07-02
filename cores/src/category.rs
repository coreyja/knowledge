use sqlx::PgPool;
use uuid::Uuid;

use crate::openai_utils::generate_embedding;

#[derive(sqlx::FromRow, Debug)]
pub struct Category {
    pub markdown_id: Uuid,
    pub category: Option<String>,
    pub embedding: Option<Vec<f64>>,
}

pub async fn store_category(
    pool: &PgPool,
    markdown_id: Uuid,
    category: &str,
) -> color_eyre::Result<Category> {
    let embedding = generate_embedding(category).await?;
    let result = sqlx::query_as!(
        Category,
        "INSERT INTO Category (markdown_id, category, embedding) VALUES ($1, $2, $3) RETURNING *",
        markdown_id,
        category.to_string(),
        &embedding
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}

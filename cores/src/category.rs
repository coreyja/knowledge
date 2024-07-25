use sqlx::PgPool;
use uuid::Uuid;

use crate::openai_utils::generate_embedding;

#[derive(sqlx::FromRow, Debug)]
pub struct Category {
    pub category_id: Uuid,
    pub category: String,
    pub embedding: Option<Vec<f64>>,
}

pub async fn store_in_category_table(
    pool: &PgPool,
    category: &str,
) -> color_eyre::Result<Category> {
    let embedding = generate_embedding(category).await?;
    let result = sqlx::query_as!(
        Category,
        "INSERT INTO categories (category_id, category, embedding) VALUES ($1, $2, $3) RETURNING *",
        Uuid::new_v4(),
        category.to_string(),
        &embedding
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}

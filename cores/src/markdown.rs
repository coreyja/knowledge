use sqlx::PgPool;
use uuid::Uuid;

use crate::category::store_in_category_table;
use crate::openai_utils::generate_categories;
use crate::page_snapshot::PageSnapShot;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Markdown {
    pub markdown_id: Uuid,
    pub page_snapshot_id: Uuid,
    pub title: Option<String>,
    pub content_md: String,
    pub summary: String,
}

pub async fn store_in_markdown_table(
    pool: &PgPool,
    page_snapshot: PageSnapShot,
) -> color_eyre::Result<Markdown> {
    let markdown_id = Uuid::new_v4();
    let content_md = html2md::parse_html(&page_snapshot.cleaned_html);
    let page_snapshot_id = page_snapshot.page_snapshot_id;

    let markdown_result = sqlx::query_as!(
        Markdown,
        "INSERT INTO Markdown (markdown_id, page_snapshot_id, content_md) VALUES ($1, $2, $3) RETURNING *",
        markdown_id,
        page_snapshot_id,
        content_md
    )
    .fetch_one(pool)
    .await?;

    let category = generate_categories(&content_md).await?;
    let category_result =
        store_in_category_table(pool, markdown_result.markdown_id, &category).await?;
    println!("Category result: {category_result:?}");

    Ok(markdown_result)
}
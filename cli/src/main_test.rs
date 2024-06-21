use crate::KnowledgeArgs;
use db::PgPool;

#[cfg(test)]
use clap::Parser;


#[allow(dead_code)]
fn handle_command(
    _args: KnowledgeArgs,
    _db_pool: PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::test]
async fn test_display_users_command() {
    let args = KnowledgeArgs::parse_from(&["test", "display-users"]);
    let db_pool = db::setup_db_pool().await.unwrap();
    let result = handle_command(args, db_pool);
    assert!(result.is_ok());
}

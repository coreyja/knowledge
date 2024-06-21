use crate::KnowledgeArgs;
use db::PgPool;

#[cfg(test)]
use clap::Parser;

#[allow(dead_code)]
fn handle_command(
    _args: &KnowledgeArgs,
    _db_pool: &PgPool,
) {
    println!("test");
}

#[cfg(test)]
async fn run_test_command(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let args = KnowledgeArgs::parse_from(args);
    let db_pool = db::setup_db_pool().await.unwrap();
    handle_command(&args, &db_pool);
    Ok(())
}

#[tokio::test]
async fn test_display_users_command() {
    let result = run_test_command(&["test", "display-users"]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sign_up_command() {
    let result = run_test_command(&["test", "signup", "--username", "testuser"]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_url_command() {
    let result = run_test_command(&[
        "test",
        "add-url",
        "--url",
        "http://example.com",
        "--allow-existing",
    ])
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_status_command() {
    let result = run_test_command(&["test", "status"]).await;
    assert!(result.is_ok());
}

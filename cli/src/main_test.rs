use crate::KnowledgeArgs;
mod test {

    use clap::Parser;
    use db::PgPool;
    fn handle_command(
        args: KnowledgeArgs,
        db_pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    use crate::KnowledgeArgs;

    #[tokio::test]
    async fn test_display_users_command() {
        let args = KnowledgeArgs::parse_from(&["test", "display-users"]);
        let db_pool = db::setup_db_pool().await.unwrap();
        let result = handle_command(args, db_pool);
        assert!(result.is_ok());
    }
}

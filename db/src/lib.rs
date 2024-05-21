pub use sqlx;
use sqlx::postgres::PgPoolOptions;
pub use sqlx::PgPool;

#[tracing::instrument(err)]
pub async fn setup_db_pool() -> color_eyre::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let mut connection = pool.acquire().await?;

    let lock = sqlx::postgres::PgAdvisoryLock::new("knowledge-db-migration-lock");
    let mut lock = lock.acquire(&mut connection).await?;

    sqlx::migrate!().run(lock.as_mut()).await?;

    lock.release_now().await?;
    tracing::info!("Migration lock unlocked");

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_db {
        use sqlx::{migrate::MigrateDatabase, Postgres, Row};
        use url::Url;

        use super::*;
        struct TestPool {
            database_url: String,
            pool: PgPool,
        }

        impl AsRef<PgPool> for TestPool {
            fn as_ref(&self) -> &PgPool {
                &self.pool
            }
        }

        impl TestPool {
            async fn teardown(self) -> color_eyre::Result<()> {
                self.pool.close().await;
                Postgres::drop_database(&self.database_url).await?;
                Ok(())
            }
        }

        async fn create_test_db_pool() -> color_eyre::Result<TestPool> {
            let original_database_url = std::env::var("DATABASE_URL").unwrap();
            let mut db_url = Url::parse(&original_database_url).unwrap();
            let database_name = format!("/knowledge_test_{}", uuid::Uuid::new_v4());
            db_url.set_path(&database_name);

            let database_url = db_url.to_string();
            Postgres::create_database(&database_url).await?;
            std::env::set_var("DATABASE_URL", &database_url);

            let pool = setup_db_pool().await?;

            Ok(TestPool { pool, database_url })
        }

        #[tokio::test]
        async fn test_create_test_db_pool() {
            let pool = create_test_db_pool().await.unwrap();

            let result = sqlx::query("SELECT 1 + 1")
                .fetch_one(pool.as_ref())
                .await
                .expect("Failed to execute query");

            assert_eq!(2, result.get::<i32, _>(0));

            let db_name_row = sqlx::query("SELECT current_database()")
                .fetch_one(pool.as_ref())
                .await
                .expect("Failed to execute query");

            let db_name: String = db_name_row.get(0);

            assert!(db_name.contains("knowledge_test_"));

            pool.teardown().await.unwrap();
        }
    }

    #[tokio::test]
    async fn setup_db_pool_doesnt_panic() {
        let _ = setup_db_pool().await.unwrap();
    }
}

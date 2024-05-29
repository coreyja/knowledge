pub use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
pub use sqlx::PgPool;
use color_eyre::Result;
use uuid::Uuid;


#[derive(sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
}

#[derive(sqlx::FromRow)]
struct UrlRecord {
    url_id: Option<Uuid>,
}

#[derive(sqlx::FromRow)]
struct ExistRecord {
    exists: Option<bool>,
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

pub async fn add_url(pool: &PgPool, url: &str, allow_existing: &bool) -> color_eyre::Result<String> {
    let exist_record = sqlx::query_as!(
        ExistRecord,
        "SELECT EXISTS(SELECT 1 FROM Page WHERE url = $1) as exists",
        url
    )
    .fetch_one(*&pool)
    .await?;

    if let Some(true) = exist_record.exists {
        if *allow_existing {
            return Ok("URL already exists and re-adding is allowed.".to_string());
        } else {
            return Err(color_eyre::eyre::eyre!("URL already exists and re-adding is not allowed."));
        }
    }
    
    let result = sqlx::query!(
        "INSERT INTO Page (url) VALUES ($1) RETURNING url",
        url
    )
    .fetch_one(pool)
    .await?;

    Ok(result.url)
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
         " 
         SELECT user_id, user_name 
         FROM Users
         ",
    ).fetch_all(pool).await?;

    Ok(users)
}

pub async fn get_username_by_id(pool: &PgPool, user_id: Uuid) -> color_eyre::Result<Option<String>> {
    let record = sqlx::query!(
        "SELECT user_name FROM Users WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.user_name))
}


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

            Ok(TestPool { database_url, pool })
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

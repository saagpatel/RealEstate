// Database test utilities

use sqlx::{sqlite::SqlitePool, sqlite::SqlitePoolOptions};

/// Creates an in-memory SQLite database with migrations applied
/// Each test gets a fresh database to prevent test interference
pub async fn create_test_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite::memory:";

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations from src-tauri/migrations/
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

/// Optional cleanup function for test databases
/// Note: In-memory databases are automatically cleaned up when pool is dropped
pub async fn cleanup_test_db(pool: &SqlitePool) {
    // Explicit cleanup if needed (though :memory: DBs auto-cleanup)
    let _ = sqlx::query("DELETE FROM listings").execute(pool).await;
    let _ = sqlx::query("DELETE FROM photos").execute(pool).await;
    let _ = sqlx::query("DELETE FROM brand_voices").execute(pool).await;
    let _ = sqlx::query("DELETE FROM properties").execute(pool).await;
    let _ = sqlx::query("DELETE FROM settings").execute(pool).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_creation() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        // Verify all tables exist by querying them
        let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM properties")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "Properties table should exist");
        assert_eq!(result.unwrap(), 0, "Properties table should be empty");
    }

    #[tokio::test]
    async fn test_db_migrations_run() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        // Verify all 5 tables were created by migrations
        let tables = vec![
            "properties",
            "photos",
            "brand_voices",
            "listings",
            "settings",
        ];

        for table in tables {
            let result = sqlx::query(&format!("SELECT COUNT(*) FROM {}", table))
                .fetch_one(&pool)
                .await;

            assert!(
                result.is_ok(),
                "Table {} should exist after migrations",
                table
            );
        }
    }

    #[tokio::test]
    async fn test_db_isolation() {
        // Create two separate test databases
        let pool1 = create_test_db().await.expect("Failed to create DB 1");
        let pool2 = create_test_db().await.expect("Failed to create DB 2");

        // Insert into first DB
        sqlx::query("INSERT INTO settings (key, value) VALUES (?, ?)")
            .bind("test_key")
            .bind("test_value")
            .execute(&pool1)
            .await
            .expect("Failed to insert");

        // Verify second DB is isolated (no data)
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM settings WHERE key = ?")
            .bind("test_key")
            .fetch_one(&pool2)
            .await
            .expect("Failed to query");

        assert_eq!(count, 0, "Second DB should not see first DB's data");
    }
}

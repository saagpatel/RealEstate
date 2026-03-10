use sqlx::{Row, SqlitePool};

use crate::error::AppError;

pub async fn get(pool: &SqlitePool, key: &str) -> Result<String, AppError> {
    let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
        .bind(key)
        .fetch_one(pool)
        .await?;
    Ok(row.get("value"))
}

pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_pool;

    #[tokio::test]
    async fn test_get_default_settings() {
        let pool = test_pool().await;
        let api_key = get(&pool, "api_key").await.unwrap();
        assert_eq!(api_key, "");

        let default_tone = get(&pool, "default_tone").await.unwrap();
        assert_eq!(default_tone, "warm");
    }

    #[tokio::test]
    async fn test_set_and_get() {
        let pool = test_pool().await;
        set(&pool, "api_key", "sk-ant-test123").await.unwrap();

        let value = get(&pool, "api_key").await.unwrap();
        assert_eq!(value, "sk-ant-test123");
    }

    #[tokio::test]
    async fn test_set_creates_new_key() {
        let pool = test_pool().await;
        set(&pool, "custom_setting", "custom_value").await.unwrap();

        let value = get(&pool, "custom_setting").await.unwrap();
        assert_eq!(value, "custom_value");
    }
}

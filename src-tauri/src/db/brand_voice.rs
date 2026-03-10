use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct BrandVoice {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub extracted_style: String,
    pub source_listings: String,
    pub sample_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    description: Option<&str>,
    extracted_style: &str,
    source_listings: &[String],
) -> Result<BrandVoice, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let source_json = serde_json::to_string(source_listings)?;
    let sample_count = source_listings.len() as i64;

    sqlx::query(
        "INSERT INTO brand_voices (id, name, description, extracted_style, source_listings, sample_count)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(description)
    .bind(extracted_style)
    .bind(&source_json)
    .bind(sample_count)
    .execute(pool)
    .await?;

    get(pool, &id).await
}

pub async fn get(pool: &SqlitePool, id: &str) -> Result<BrandVoice, AppError> {
    let voice = sqlx::query_as::<_, BrandVoice>(
        "SELECT id, name, description, extracted_style, source_listings, sample_count, created_at, updated_at FROM brand_voices WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(voice)
}

pub async fn list_all(pool: &SqlitePool) -> Result<Vec<BrandVoice>, AppError> {
    let voices = sqlx::query_as::<_, BrandVoice>(
        "SELECT id, name, description, extracted_style, source_listings, sample_count, created_at, updated_at FROM brand_voices ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(voices)
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM brand_voices WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_pool;

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = test_pool().await;
        let style = r#"{"tone":"warm","vocabulary":["stunning"]}"#;
        let sources = vec![
            "Sample listing 1".to_string(),
            "Sample listing 2".to_string(),
        ];

        let voice = create(&pool, "My Style", Some("Test voice"), style, &sources)
            .await
            .unwrap();

        assert_eq!(voice.name, "My Style");
        assert_eq!(voice.sample_count, 2);

        let fetched = get(&pool, &voice.id).await.unwrap();
        assert_eq!(fetched.id, voice.id);
    }

    #[tokio::test]
    async fn test_list_and_delete() {
        let pool = test_pool().await;
        let style = r#"{"tone":"professional"}"#;

        create(&pool, "Voice 1", None, style, &["sample".to_string()])
            .await
            .unwrap();
        let v2 = create(&pool, "Voice 2", None, style, &["sample".to_string()])
            .await
            .unwrap();

        let all = list_all(&pool).await.unwrap();
        assert_eq!(all.len(), 2);

        delete(&pool, &v2.id).await.unwrap();
        let all = list_all(&pool).await.unwrap();
        assert_eq!(all.len(), 1);
    }
}

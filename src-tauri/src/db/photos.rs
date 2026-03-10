use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: String,
    pub property_id: String,
    pub filename: String,
    pub original_path: String,
    pub thumbnail_path: String,
    pub sort_order: i64,
    pub caption: Option<String>,
    pub created_at: String,
}

pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    property_id: &str,
    filename: &str,
    original_path: &str,
    thumbnail_path: &str,
    sort_order: i64,
) -> Result<Photo, AppError> {
    sqlx::query(
        "INSERT INTO photos (id, property_id, filename, original_path, thumbnail_path, sort_order)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(property_id)
    .bind(filename)
    .bind(original_path)
    .bind(thumbnail_path)
    .bind(sort_order)
    .execute(pool)
    .await?;

    get(pool, id).await
}

pub async fn list_by_property(
    pool: &SqlitePool,
    property_id: &str,
) -> Result<Vec<Photo>, AppError> {
    let photos = sqlx::query_as::<_, Photo>(
        "SELECT id, property_id, filename, original_path, thumbnail_path, sort_order, caption, created_at
         FROM photos WHERE property_id = ? ORDER BY sort_order ASC",
    )
    .bind(property_id)
    .fetch_all(pool)
    .await?;

    Ok(photos)
}

pub async fn get(pool: &SqlitePool, id: &str) -> Result<Photo, AppError> {
    let photo = sqlx::query_as::<_, Photo>(
        "SELECT id, property_id, filename, original_path, thumbnail_path, sort_order, caption, created_at
         FROM photos WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(photo)
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM photos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_sort_order(
    pool: &SqlitePool,
    id: &str,
    sort_order: i64,
) -> Result<(), AppError> {
    sqlx::query("UPDATE photos SET sort_order = ? WHERE id = ?")
        .bind(sort_order)
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
    async fn test_insert_and_get() {
        let pool = test_pool().await;

        // Need a property first due to FK constraint
        sqlx::query(
            "INSERT INTO properties (id, address, city, state, zip, beds, baths, sqft, price, property_type, key_features, neighborhood_highlights, nearby_amenities)
             VALUES ('prop1', '123 Main', 'City', 'ST', '12345', 3, 2.0, 1500, 50000000, 'single_family', '[]', '[]', '[]')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let photo = insert(
            &pool,
            "photo1",
            "prop1",
            "image.jpg",
            "/path/to/original.jpg",
            "/path/to/thumb.jpg",
            0,
        )
        .await
        .unwrap();

        assert_eq!(photo.id, "photo1");
        assert_eq!(photo.filename, "image.jpg");
        assert_eq!(photo.sort_order, 0);

        let fetched = get(&pool, "photo1").await.unwrap();
        assert_eq!(fetched.id, photo.id);
    }

    #[tokio::test]
    async fn test_list_by_property() {
        let pool = test_pool().await;

        sqlx::query(
            "INSERT INTO properties (id, address, city, state, zip, beds, baths, sqft, price, property_type, key_features, neighborhood_highlights, nearby_amenities)
             VALUES ('prop1', '123 Main', 'City', 'ST', '12345', 3, 2.0, 1500, 50000000, 'single_family', '[]', '[]', '[]')"
        )
        .execute(&pool)
        .await
        .unwrap();

        insert(&pool, "p1", "prop1", "a.jpg", "/a", "/ta", 1)
            .await
            .unwrap();
        insert(&pool, "p2", "prop1", "b.jpg", "/b", "/tb", 0)
            .await
            .unwrap();

        let photos = list_by_property(&pool, "prop1").await.unwrap();
        assert_eq!(photos.len(), 2);
        // Should be ordered by sort_order ASC
        assert_eq!(photos[0].id, "p2");
        assert_eq!(photos[1].id, "p1");
    }

    #[tokio::test]
    async fn test_delete() {
        let pool = test_pool().await;

        sqlx::query(
            "INSERT INTO properties (id, address, city, state, zip, beds, baths, sqft, price, property_type, key_features, neighborhood_highlights, nearby_amenities)
             VALUES ('prop1', '123 Main', 'City', 'ST', '12345', 3, 2.0, 1500, 50000000, 'single_family', '[]', '[]', '[]')"
        )
        .execute(&pool)
        .await
        .unwrap();

        insert(&pool, "p1", "prop1", "a.jpg", "/a", "/ta", 0)
            .await
            .unwrap();
        delete(&pool, "p1").await.unwrap();

        let result = get(&pool, "p1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_sort_order() {
        let pool = test_pool().await;

        sqlx::query(
            "INSERT INTO properties (id, address, city, state, zip, beds, baths, sqft, price, property_type, key_features, neighborhood_highlights, nearby_amenities)
             VALUES ('prop1', '123 Main', 'City', 'ST', '12345', 3, 2.0, 1500, 50000000, 'single_family', '[]', '[]', '[]')"
        )
        .execute(&pool)
        .await
        .unwrap();

        insert(&pool, "p1", "prop1", "a.jpg", "/a", "/ta", 0)
            .await
            .unwrap();
        update_sort_order(&pool, "p1", 5).await.unwrap();

        let photo = get(&pool, "p1").await.unwrap();
        assert_eq!(photo.sort_order, 5);
    }
}

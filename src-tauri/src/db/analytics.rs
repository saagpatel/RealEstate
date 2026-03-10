use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;

pub struct GenerationAnalyticsRecord<'a> {
    pub property_id: &'a str,
    pub generation_type: &'a str,
    pub model_used: &'a str,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_cents: u32,
    pub latency_ms: u64,
    pub success: bool,
    pub error_message: Option<&'a str>,
}

/// Record a generation event for analytics
pub async fn record_generation(
    db: &SqlitePool,
    record: GenerationAnalyticsRecord<'_>,
) -> Result<(), AppError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO generation_analytics (
            id, property_id, generation_type, model_used,
            input_tokens, output_tokens, cost_cents, latency_ms,
            success, error_message
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(record.property_id)
    .bind(record.generation_type)
    .bind(record.model_used)
    .bind(i64::from(record.input_tokens))
    .bind(i64::from(record.output_tokens))
    .bind(i64::from(record.cost_cents))
    .bind(record.latency_ms as i64)
    .bind(record.success)
    .bind(record.error_message)
    .execute(db)
    .await?;

    Ok(())
}

/// Record an export event for analytics
pub async fn record_export(
    db: &SqlitePool,
    property_id: &str,
    export_format: &str,
    listing_count: usize,
    photo_count: usize,
    file_size_bytes: usize,
    generation_time_ms: u64,
) -> Result<(), AppError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO export_analytics (
            id, property_id, export_format, listing_count,
            photo_count, file_size_bytes, generation_time_ms
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(property_id)
    .bind(export_format)
    .bind(listing_count as i64)
    .bind(photo_count as i64)
    .bind(file_size_bytes as i64)
    .bind(generation_time_ms as i64)
    .execute(db)
    .await?;

    Ok(())
}

/// Get total generations count
pub async fn get_total_generations(db: &SqlitePool) -> Result<i64, AppError> {
    let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM generation_analytics")
        .fetch_one(db)
        .await?;

    Ok(result)
}

/// Get total cost in cents
pub async fn get_total_cost(db: &SqlitePool) -> Result<i64, AppError> {
    let result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(SUM(cost_cents), 0)
        FROM generation_analytics
        WHERE success = 1
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok(result)
}

/// Get average latency in milliseconds
pub async fn get_average_latency(db: &SqlitePool) -> Result<f64, AppError> {
    let result = sqlx::query_scalar::<_, f64>(
        r#"
        SELECT COALESCE(AVG(latency_ms), 0.0)
        FROM generation_analytics
        WHERE success = 1
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok(result)
}

/// Get success rate as a percentage
pub async fn get_success_rate(db: &SqlitePool) -> Result<f64, AppError> {
    let result = sqlx::query(
        r#"
        SELECT
            COUNT(*) as total,
            COALESCE(SUM(success), 0) as successful
        FROM generation_analytics
        "#,
    )
    .fetch_one(db)
    .await?;

    let total: i64 = result.get("total");
    if total == 0 {
        return Ok(100.0);
    }

    let successful: i64 = result.get("successful");
    let rate = (successful as f64 / total as f64) * 100.0;
    Ok(rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::properties::{self, CreatePropertyInput};

    async fn setup_test_db() -> SqlitePool {
        let db = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&db).await.unwrap();
        db
    }

    async fn create_test_property(db: &SqlitePool, address: &str) -> String {
        let property = properties::create(
            db,
            CreatePropertyInput {
                address: address.to_string(),
                city: "San Francisco".to_string(),
                state: "CA".to_string(),
                zip: "94105".to_string(),
                beds: 3,
                baths: 2.0,
                sqft: 1800,
                price: 95000000,
                property_type: "single_family".to_string(),
                year_built: Some(2015),
                lot_size: Some("5000 sqft".to_string()),
                parking: Some("Garage".to_string()),
                key_features: vec!["Updated kitchen".to_string()],
                neighborhood: Some("Mission Bay".to_string()),
                neighborhood_highlights: vec!["Near transit".to_string()],
                school_district: Some("SFUSD".to_string()),
                nearby_amenities: vec!["Parks".to_string()],
                agent_notes: Some("Test fixture".to_string()),
            },
        )
        .await
        .unwrap();

        property.id
    }

    #[tokio::test]
    async fn test_record_generation() {
        let db = setup_test_db().await;
        let property_id = create_test_property(&db, "123 Test Street").await;

        let result = record_generation(
            &db,
            GenerationAnalyticsRecord {
                property_id: &property_id,
                generation_type: "listing",
                model_used: "claude-sonnet-4-5",
                input_tokens: 1000,
                output_tokens: 2000,
                cost_cents: 5,
                latency_ms: 1500,
                success: true,
                error_message: None,
            },
        )
        .await;

        assert!(result.is_ok());

        let count = get_total_generations(&db).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_get_total_cost() {
        let db = setup_test_db().await;
        let property_one = create_test_property(&db, "123 Test Street").await;
        let property_two = create_test_property(&db, "456 Test Street").await;

        record_generation(
            &db,
            GenerationAnalyticsRecord {
                property_id: &property_one,
                generation_type: "listing",
                model_used: "sonnet",
                input_tokens: 100,
                output_tokens: 200,
                cost_cents: 10,
                latency_ms: 1000,
                success: true,
                error_message: None,
            },
        )
        .await
        .unwrap();
        record_generation(
            &db,
            GenerationAnalyticsRecord {
                property_id: &property_two,
                generation_type: "social",
                model_used: "sonnet",
                input_tokens: 100,
                output_tokens: 200,
                cost_cents: 15,
                latency_ms: 1200,
                success: true,
                error_message: None,
            },
        )
        .await
        .unwrap();

        let total = get_total_cost(&db).await.unwrap();
        assert_eq!(total, 25);
    }

    #[tokio::test]
    async fn test_success_rate() {
        let db = setup_test_db().await;
        let property_one = create_test_property(&db, "123 Test Street").await;
        let property_two = create_test_property(&db, "456 Test Street").await;

        record_generation(
            &db,
            GenerationAnalyticsRecord {
                property_id: &property_one,
                generation_type: "listing",
                model_used: "sonnet",
                input_tokens: 100,
                output_tokens: 200,
                cost_cents: 10,
                latency_ms: 1000,
                success: true,
                error_message: None,
            },
        )
        .await
        .unwrap();
        record_generation(
            &db,
            GenerationAnalyticsRecord {
                property_id: &property_two,
                generation_type: "listing",
                model_used: "sonnet",
                input_tokens: 100,
                output_tokens: 200,
                cost_cents: 10,
                latency_ms: 1000,
                success: false,
                error_message: Some("API error"),
            },
        )
        .await
        .unwrap();

        let rate = get_success_rate(&db).await.unwrap();
        assert_eq!(rate, 50.0);
    }
}

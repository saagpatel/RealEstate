// Property command handler tests
// These test the Tauri command layer (thin wrappers around db functions)

#[cfg(test)]
mod tests {
    use crate::common::db_setup::create_test_db;
    use realestate_lib::db::properties::CreatePropertyInput;

    fn sample_property_input() -> CreatePropertyInput {
        CreatePropertyInput {
            address: "456 Test Ave".to_string(),
            city: "Portland".to_string(),
            state: "OR".to_string(),
            zip: "97201".to_string(),
            beds: 4,
            baths: 3.0,
            sqft: 2800,
            price: 75_000_000, // Price in cents
            property_type: "single_family".to_string(),
            year_built: Some(2018),
            lot_size: Some("6000 sqft".to_string()),
            parking: Some("3-car garage".to_string()),
            key_features: vec!["Gourmet kitchen".to_string(), "Master suite".to_string()],
            neighborhood: Some("Pearl District".to_string()),
            neighborhood_highlights: vec!["Walk Score 98".to_string()],
            school_district: Some("Portland Public Schools".to_string()),
            nearby_amenities: vec!["Trader Joe's 0.2mi".to_string()],
            agent_notes: Some("Great investment".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_property_command() {
        let pool = create_test_db().await.expect("Failed to create test DB");
        let input = sample_property_input();

        // Note: Command handlers expect tauri::State, but for testing we can call
        // the underlying db functions directly since commands are thin wrappers
        let property = realestate_lib::db::properties::create(&pool, input)
            .await
            .expect("Failed to create property");

        assert_eq!(property.address, "456 Test Ave");
        assert_eq!(property.city, "Portland");
        assert_eq!(property.beds, 4);
        assert_eq!(property.baths, 3.0);
        assert_eq!(property.price, 75_000_000);
        assert!(!property.id.is_empty());
    }

    #[tokio::test]
    async fn test_get_property_command() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let created = realestate_lib::db::properties::create(&pool, sample_property_input())
            .await
            .expect("Failed to create");

        let fetched = realestate_lib::db::properties::get(&pool, &created.id)
            .await
            .expect("Failed to get property");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.address, "456 Test Ave");
    }

    #[tokio::test]
    async fn test_list_properties_command() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        // Create multiple properties
        for i in 0..5 {
            let mut input = sample_property_input();
            input.address = format!("{} Main St", i);
            realestate_lib::db::properties::create(&pool, input)
                .await
                .expect("Failed to create");
        }

        let properties = realestate_lib::db::properties::list_all(&pool)
            .await
            .expect("Failed to list");

        assert_eq!(properties.len(), 5);
    }

    #[tokio::test]
    async fn test_update_property_command() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let created = realestate_lib::db::properties::create(&pool, sample_property_input())
            .await
            .expect("Failed to create");

        let mut updated_input = sample_property_input();
        updated_input.address = "999 Updated Blvd".to_string();
        updated_input.price = 85_000_000;
        updated_input.beds = 5;

        let updated = realestate_lib::db::properties::update(&pool, &created.id, updated_input)
            .await
            .expect("Failed to update");

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.address, "999 Updated Blvd");
        assert_eq!(updated.price, 85_000_000);
        assert_eq!(updated.beds, 5);
    }

    #[tokio::test]
    async fn test_delete_property_command() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let created = realestate_lib::db::properties::create(&pool, sample_property_input())
            .await
            .expect("Failed to create");

        realestate_lib::db::properties::delete(&pool, &created.id)
            .await
            .expect("Failed to delete");

        // Verify deletion
        let result = realestate_lib::db::properties::get(&pool, &created.id).await;
        assert!(result.is_err(), "Property should be deleted");
    }

    #[tokio::test]
    async fn test_property_with_minimal_fields() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let minimal_input = CreatePropertyInput {
            address: "Minimal Property".to_string(),
            city: "City".to_string(),
            state: "ST".to_string(),
            zip: "12345".to_string(),
            beds: 2,
            baths: 1.0,
            sqft: 1000,
            price: 30_000_000,
            property_type: "condo".to_string(),
            year_built: None,
            lot_size: None,
            parking: None,
            key_features: vec![],
            neighborhood: None,
            neighborhood_highlights: vec![],
            school_district: None,
            nearby_amenities: vec![],
            agent_notes: None,
        };

        let property = realestate_lib::db::properties::create(&pool, minimal_input)
            .await
            .expect("Failed to create minimal property");

        assert_eq!(property.address, "Minimal Property");
        assert!(property.year_built.is_none());
        assert!(property.lot_size.is_none());
    }

    #[tokio::test]
    async fn test_property_json_array_fields() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let mut input = sample_property_input();
        input.key_features = vec![
            "Feature 1".to_string(),
            "Feature 2".to_string(),
            "Feature 3".to_string(),
        ];
        input.nearby_amenities = vec!["Amenity A".to_string(), "Amenity B".to_string()];

        let property = realestate_lib::db::properties::create(&pool, input)
            .await
            .expect("Failed to create");

        // JSON fields are serialized and deserialized correctly
        assert!(!property.key_features.is_empty());
        assert!(!property.nearby_amenities.is_empty());
    }
}

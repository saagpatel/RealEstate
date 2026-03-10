// End-to-end integration tests
// Tests full workflows from property creation through generation to export

#[cfg(test)]
mod tests {
    use crate::common::db_setup::create_test_db;
    use realestate_lib::db::brand_voice;
    use realestate_lib::db::listings::{self, CreateListingInput};
    use realestate_lib::db::photos;
    use realestate_lib::db::properties::{self, CreatePropertyInput};
    use realestate_lib::db::settings;

    fn sample_property_input() -> CreatePropertyInput {
        CreatePropertyInput {
            address: "789 Integration Test Ln".to_string(),
            city: "Seattle".to_string(),
            state: "WA".to_string(),
            zip: "98101".to_string(),
            beds: 3,
            baths: 2.5,
            sqft: 2200,
            price: 92_500_000,
            property_type: "townhouse".to_string(),
            year_built: Some(2020),
            lot_size: Some("2500 sqft".to_string()),
            parking: Some("2-car attached".to_string()),
            key_features: vec!["Open floor plan".to_string(), "Rooftop deck".to_string()],
            neighborhood: Some("Capitol Hill".to_string()),
            neighborhood_highlights: vec!["Walk Score 95".to_string()],
            school_district: Some("Seattle Public Schools".to_string()),
            nearby_amenities: vec!["Light rail 0.1mi".to_string()],
            agent_notes: Some("Hot market".to_string()),
        }
    }

    #[tokio::test]
    async fn test_full_property_lifecycle() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        // Step 1: Create property
        let property = properties::create(&pool, sample_property_input())
            .await
            .expect("Failed to create property");

        assert_eq!(property.address, "789 Integration Test Ln");
        assert!(!property.id.is_empty());

        // Step 2: Add photos
        let photo1 = photos::insert(
            &pool,
            "photo-front",
            &property.id,
            "front_exterior.jpg",
            "/tmp/front_exterior.jpg",
            "/tmp/thumb_front.jpg",
            0,
        )
        .await
        .expect("Failed to add photo");

        let photo2 = photos::insert(
            &pool,
            "photo-kitchen",
            &property.id,
            "kitchen.jpg",
            "/tmp/kitchen.jpg",
            "/tmp/thumb_kitchen.jpg",
            1,
        )
        .await
        .expect("Failed to add photo");

        // Step 3: List photos
        let all_photos = photos::list_by_property(&pool, &property.id)
            .await
            .expect("Failed to list photos");

        assert_eq!(all_photos.len(), 2);

        // Step 4: Create brand voice
        let voice = brand_voice::create(
            &pool,
            "Seattle Luxury Voice",
            Some("For upscale Seattle properties"),
            "Sophisticated yet approachable",
            &[
                "Upscale Seattle property with strong buyer appeal".to_string(),
                "Luxury townhome copy with neighborhood context".to_string(),
            ],
        )
        .await
        .expect("Failed to create brand voice");

        // Step 5: Set API key in settings
        settings::set(&pool, "api_key", "sk-ant-test-integration")
            .await
            .expect("Failed to set API key");

        let api_key = settings::get(&pool, "api_key")
            .await
            .expect("Failed to get API key");

        assert_eq!(api_key, "sk-ant-test-integration");

        // Step 6: Save a listing (simulating AI generation result)
        let listing = listings::save(
            &pool,
            CreateListingInput {
                property_id: property.id.clone(),
                content: "This stunning townhouse in Capitol Hill features an open floor plan and rooftop deck with city views. Recently built in 2020, it offers modern finishes throughout.".to_string(),
                generation_type: "listing".to_string(),
                style: Some("luxury".to_string()),
                tone: Some("warm".to_string()),
                length: Some("medium".to_string()),
                seo_keywords: vec!["seattle".to_string(), "townhouse".to_string(), "capitol hill".to_string()],
                brand_voice_id: Some(voice.id.clone()),
                tokens_used: 485,
                generation_cost_cents: 6,
            },
        )
        .await
        .expect("Failed to save listing");

        assert!(!listing.id.is_empty());
        assert_eq!(listing.property_id, property.id);
        assert_eq!(listing.tokens_used, 485);

        // Step 7: Toggle listing favorite
        listings::toggle_favorite(&pool, &listing.id)
            .await
            .expect("Failed to toggle favorite");

        let favorited = listings::get(&pool, &listing.id)
            .await
            .expect("Failed to get listing");

        assert!(favorited.is_favorite);

        // Step 8: List all listings for property
        let all_listings = listings::list_by_property(&pool, &property.id)
            .await
            .expect("Failed to list listings");

        assert_eq!(all_listings.len(), 1);
        assert_eq!(all_listings[0].id, listing.id);

        // Step 9: Update property
        let mut updated_input = sample_property_input();
        updated_input.price = 95_000_000;
        updated_input.agent_notes = Some("Price updated".to_string());

        let updated_property = properties::update(&pool, &property.id, updated_input)
            .await
            .expect("Failed to update property");

        assert_eq!(updated_property.price, 95_000_000);

        // Step 10: Cleanup - delete listing first (foreign key constraint)
        listings::delete(&pool, &listing.id)
            .await
            .expect("Failed to delete listing");

        // Delete photos
        photos::delete(&pool, &photo1.id)
            .await
            .expect("Failed to delete photo1");
        photos::delete(&pool, &photo2.id)
            .await
            .expect("Failed to delete photo2");

        // Delete brand voice
        brand_voice::delete(&pool, &voice.id)
            .await
            .expect("Failed to delete voice");

        // Finally delete property
        properties::delete(&pool, &property.id)
            .await
            .expect("Failed to delete property");

        // Verify property is deleted
        let result = properties::get(&pool, &property.id).await;
        assert!(result.is_err(), "Property should be deleted");
    }

    #[tokio::test]
    async fn test_multiple_properties_with_listings() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let mut property_ids = vec![];

        // Create 3 properties
        for i in 0..3 {
            let mut input = sample_property_input();
            input.address = format!("{} Test Property Ln", i);
            input.price = 80_000_000 + (i * 5_000_000);

            let property = properties::create(&pool, input)
                .await
                .expect("Failed to create property");

            property_ids.push(property.id.clone());

            // Create 2 listings per property
            for j in 0..2 {
                listings::save(
                    &pool,
                    CreateListingInput {
                        property_id: property.id.clone(),
                        content: format!("Listing {} for property {}", j, i),
                        generation_type: if j == 0 {
                            "listing"
                        } else {
                            "social_instagram"
                        }
                        .to_string(),
                        style: Some("luxury".to_string()),
                        tone: None,
                        length: None,
                        seo_keywords: vec![],
                        brand_voice_id: None,
                        tokens_used: 400 + (j * 50),
                        generation_cost_cents: 5,
                    },
                )
                .await
                .expect("Failed to save listing");
            }
        }

        // Verify all properties
        let all_properties = properties::list_all(&pool)
            .await
            .expect("Failed to list properties");

        assert_eq!(all_properties.len(), 3);

        // Verify listings for each property
        for property_id in &property_ids {
            let prop_listings = listings::list_by_property(&pool, property_id)
                .await
                .expect("Failed to list listings");

            assert_eq!(prop_listings.len(), 2);
            assert!(prop_listings.iter().any(|l| l.generation_type == "listing"));
            assert!(prop_listings
                .iter()
                .any(|l| l.generation_type == "social_instagram"));
        }
    }

    #[tokio::test]
    async fn test_property_without_optional_fields() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let minimal_input = CreatePropertyInput {
            address: "Minimal Test".to_string(),
            city: "City".to_string(),
            state: "ST".to_string(),
            zip: "12345".to_string(),
            beds: 1,
            baths: 1.0,
            sqft: 800,
            price: 25_000_000,
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

        let property = properties::create(&pool, minimal_input)
            .await
            .expect("Failed to create minimal property");

        // Create listing for minimal property
        let listing = listings::save(
            &pool,
            CreateListingInput {
                property_id: property.id.clone(),
                content: "Cozy studio".to_string(),
                generation_type: "listing".to_string(),
                style: None,
                tone: None,
                length: None,
                seo_keywords: vec![],
                brand_voice_id: None,
                tokens_used: 200,
                generation_cost_cents: 3,
            },
        )
        .await
        .expect("Failed to save listing");

        assert!(!listing.id.is_empty());
    }

    #[tokio::test]
    async fn test_photo_reordering() {
        let pool = create_test_db().await.expect("Failed to create test DB");

        let property = properties::create(&pool, sample_property_input())
            .await
            .expect("Failed to create property");

        // Add 4 photos
        for i in 0..4 {
            photos::insert(
                &pool,
                &format!("photo-{}", i),
                &property.id,
                &format!("photo_{}.jpg", i),
                &format!("/tmp/photo_{}.jpg", i),
                &format!("/tmp/thumb_{}.jpg", i),
                i as i64,
            )
            .await
            .expect("Failed to add photo");
        }

        let initial_photos = photos::list_by_property(&pool, &property.id)
            .await
            .expect("Failed to list photos");

        assert_eq!(initial_photos.len(), 4);

        // Reorder: reverse the order
        for (index, photo) in initial_photos.iter().rev().enumerate() {
            photos::update_sort_order(&pool, &photo.id, index as i64)
                .await
                .expect("Failed to reorder");
        }

        let reordered = photos::list_by_property(&pool, &property.id)
            .await
            .expect("Failed to list reordered photos");

        assert_eq!(reordered[0].filename, "photo_3.jpg");
        assert_eq!(reordered[3].filename, "photo_0.jpg");
    }
}

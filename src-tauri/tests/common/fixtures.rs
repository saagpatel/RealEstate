// Test data builders (fixtures) for creating test entities

/// Builder for creating test Property instances
#[derive(Debug, Clone)]
pub struct PropertyBuilder {
    pub id: i64,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub beds: i32,
    pub baths: i32,
    pub sqft: Option<i32>,
    pub price: i64,
    pub property_type: String,
    pub year_built: Option<i32>,
    pub lot_size: Option<String>,
    pub parking: Option<String>,
    pub key_features: Vec<String>,
    pub highlights: Vec<String>,
    pub amenities: Vec<String>,
}

impl PropertyBuilder {
    /// Creates a default test property
    pub fn new() -> Self {
        PropertyBuilder {
            id: 1,
            address: "123 Main St".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip: "94102".to_string(),
            beds: 3,
            baths: 2,
            sqft: Some(2500),
            price: 85_000_000, // Price in cents
            property_type: "single_family".to_string(),
            year_built: Some(2010),
            lot_size: Some("5000 sqft".to_string()),
            parking: Some("2-car garage".to_string()),
            key_features: vec!["Updated kitchen".to_string(), "Hardwood floors".to_string()],
            highlights: vec!["New roof".to_string()],
            amenities: vec!["Fireplace".to_string(), "Deck".to_string()],
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn with_address(mut self, addr: &str) -> Self {
        self.address = addr.to_string();
        self
    }

    pub fn with_city(mut self, city: &str) -> Self {
        self.city = city.to_string();
        self
    }

    pub fn with_state(mut self, state: &str) -> Self {
        self.state = state.to_string();
        self
    }

    pub fn with_zip(mut self, zip: &str) -> Self {
        self.zip = zip.to_string();
        self
    }

    pub fn with_price(mut self, price: i64) -> Self {
        self.price = price;
        self
    }

    pub fn with_beds(mut self, beds: i32) -> Self {
        self.beds = beds;
        self
    }

    pub fn with_baths(mut self, baths: i32) -> Self {
        self.baths = baths;
        self
    }

    pub fn with_sqft(mut self, sqft: i32) -> Self {
        self.sqft = Some(sqft);
        self
    }

    pub fn with_property_type(mut self, property_type: &str) -> Self {
        self.property_type = property_type.to_string();
        self
    }
}

impl Default for PropertyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test Listing instances
#[derive(Debug, Clone)]
pub struct ListingBuilder {
    pub id: i64,
    pub property_id: i64,
    pub content: String,
    pub generation_type: String,
    pub style: Option<String>,
    pub tone: Option<String>,
    pub length: Option<String>,
    pub seo_keywords: Option<String>,
    pub tokens_used: i32,
    pub cost_cents: i32,
    pub is_favorite: bool,
}

impl ListingBuilder {
    /// Creates a default test listing for a property
    pub fn new(property_id: i64) -> Self {
        ListingBuilder {
            id: 1,
            property_id,
            content: "Beautiful home in a great neighborhood with modern finishes.".to_string(),
            generation_type: "listing".to_string(),
            style: Some("Professional".to_string()),
            tone: Some("Friendly".to_string()),
            length: Some("Medium".to_string()),
            seo_keywords: Some("modern,updated,neighborhood".to_string()),
            tokens_used: 450,
            cost_cents: 5,
            is_favorite: false,
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn with_generation_type(mut self, gen_type: &str) -> Self {
        self.generation_type = gen_type.to_string();
        self
    }

    pub fn with_style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    pub fn with_tone(mut self, tone: &str) -> Self {
        self.tone = Some(tone.to_string());
        self
    }

    pub fn with_length(mut self, length: &str) -> Self {
        self.length = Some(length.to_string());
        self
    }

    pub fn with_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }
}

/// Builder for creating test Photo instances
#[derive(Debug, Clone)]
pub struct PhotoBuilder {
    pub id: i64,
    pub property_id: i64,
    pub filename: String,
    pub thumbnail_path: Option<String>,
    pub sort_order: i32,
    pub caption: Option<String>,
}

impl PhotoBuilder {
    /// Creates a default test photo for a property
    pub fn new(property_id: i64) -> Self {
        PhotoBuilder {
            id: 1,
            property_id,
            filename: "front_view.jpg".to_string(),
            thumbnail_path: Some("thumb_front_view.jpg".to_string()),
            sort_order: 0,
            caption: Some("Front exterior view".to_string()),
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    pub fn with_sort_order(mut self, order: i32) -> Self {
        self.sort_order = order;
        self
    }

    pub fn without_thumbnail(mut self) -> Self {
        self.thumbnail_path = None;
        self
    }
}

/// Builder for creating test BrandVoice instances
#[derive(Debug, Clone)]
pub struct BrandVoiceBuilder {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub extracted_style: Option<String>,
    pub source_listings: Vec<String>,
    pub sample_count: i32,
}

impl BrandVoiceBuilder {
    /// Creates a default test brand voice
    pub fn new() -> Self {
        BrandVoiceBuilder {
            id: 1,
            name: "Luxury Voice".to_string(),
            description: Some("For upscale properties".to_string()),
            extracted_style: Some(
                "Sophisticated and elegant tone with emphasis on premium features".to_string(),
            ),
            source_listings: vec![
                "Sample listing 1 with luxury language".to_string(),
                "Sample listing 2 with upscale terminology".to_string(),
            ],
            sample_count: 2,
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_sample_count(mut self, count: i32) -> Self {
        self.sample_count = count;
        self
    }
}

impl Default for BrandVoiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_builder_defaults() {
        let property = PropertyBuilder::new();
        assert_eq!(property.address, "123 Main St");
        assert_eq!(property.city, "San Francisco");
        assert_eq!(property.beds, 3);
        assert_eq!(property.baths, 2);
    }

    #[test]
    fn test_property_builder_with_methods() {
        let property = PropertyBuilder::new()
            .with_address("456 Oak Ave")
            .with_price(95_000_000)
            .with_beds(4);

        assert_eq!(property.address, "456 Oak Ave");
        assert_eq!(property.price, 95_000_000);
        assert_eq!(property.beds, 4);
    }

    #[test]
    fn test_listing_builder() {
        let listing = ListingBuilder::new(1)
            .with_content("Custom listing content")
            .with_favorite(true);

        assert_eq!(listing.property_id, 1);
        assert_eq!(listing.content, "Custom listing content");
        assert!(listing.is_favorite);
    }

    #[test]
    fn test_photo_builder() {
        let photo = PhotoBuilder::new(1)
            .with_filename("kitchen.jpg")
            .with_sort_order(2);

        assert_eq!(photo.property_id, 1);
        assert_eq!(photo.filename, "kitchen.jpg");
        assert_eq!(photo.sort_order, 2);
    }

    #[test]
    fn test_brand_voice_builder() {
        let voice = BrandVoiceBuilder::new()
            .with_name("Budget Voice")
            .with_sample_count(3);

        assert_eq!(voice.name, "Budget Voice");
        assert_eq!(voice.sample_count, 3);
    }
}

use csv::Reader;
use sqlx::SqlitePool;
use std::io::Cursor;

use crate::db::properties::{self, CreatePropertyInput};
use crate::error::AppError;

/// CSV row structure matching expected import format
#[derive(Debug, serde::Deserialize)]
pub struct PropertyCsvRow {
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub beds: i64,
    pub baths: f64,
    pub sqft: i64,
    pub price: i64, // in cents
    pub property_type: String,
    #[serde(default)]
    pub year_built: Option<i64>,
    #[serde(default)]
    pub lot_size: Option<String>,
    #[serde(default)]
    pub parking: Option<String>,
    #[serde(default)]
    pub key_features: String, // JSON array or empty
    #[serde(default)]
    pub neighborhood: Option<String>,
    #[serde(default)]
    pub neighborhood_highlights: String, // JSON array or empty
    #[serde(default)]
    pub school_district: Option<String>,
    #[serde(default)]
    pub nearby_amenities: String, // JSON array or empty
    #[serde(default)]
    pub agent_notes: Option<String>,
}

pub struct ImportResult {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<ImportError>,
}

#[derive(Debug, Clone)]
pub struct ImportError {
    pub row_number: usize,
    pub address: String,
    pub error: String,
}

fn parse_json_array_field(
    value: &str,
    field_name: &str,
    row_number: usize,
    address: &str,
) -> Result<Vec<String>, ImportError> {
    let normalized = if value.trim().is_empty() { "[]" } else { value };
    serde_json::from_str::<Vec<String>>(normalized).map_err(|_| ImportError {
        row_number,
        address: address.to_string(),
        error: format!("{field_name} must be valid JSON array"),
    })
}

/// Import properties from CSV data
pub async fn import_from_csv(db: &SqlitePool, csv_data: &str) -> Result<ImportResult, AppError> {
    let cursor = Cursor::new(csv_data);
    let mut reader = Reader::from_reader(cursor);

    let mut total = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    for (idx, result) in reader.deserialize().enumerate() {
        total += 1;
        let row_number = idx + 2; // +2 because CSV is 1-indexed and has header

        match result {
            Ok(row) => match import_single_property(db, row, row_number).await {
                Ok(_) => successful += 1,
                Err(e) => {
                    failed += 1;
                    errors.push(e);
                }
            },
            Err(e) => {
                failed += 1;
                errors.push(ImportError {
                    row_number,
                    address: "Unknown".to_string(),
                    error: format!("CSV parse error: {}", e),
                });
            }
        }
    }

    Ok(ImportResult {
        total,
        successful,
        failed,
        errors,
    })
}

/// Import a single property from a CSV row
async fn import_single_property(
    db: &SqlitePool,
    row: PropertyCsvRow,
    row_number: usize,
) -> Result<(), ImportError> {
    // Validate property type
    let valid_types = [
        "single_family",
        "condo",
        "townhouse",
        "multi_family",
        "land",
        "commercial",
    ];
    if !valid_types.contains(&row.property_type.as_str()) {
        return Err(ImportError {
            row_number,
            address: row.address.clone(),
            error: format!(
                "Invalid property_type '{}'. Must be one of: {}",
                row.property_type,
                valid_types.join(", ")
            ),
        });
    }

    // Validate positive values
    if row.beds < 0 {
        return Err(ImportError {
            row_number,
            address: row.address.clone(),
            error: "beds must be >= 0".to_string(),
        });
    }

    if row.baths < 0.0 {
        return Err(ImportError {
            row_number,
            address: row.address.clone(),
            error: "baths must be >= 0".to_string(),
        });
    }

    if row.sqft <= 0 {
        return Err(ImportError {
            row_number,
            address: row.address.clone(),
            error: "sqft must be > 0".to_string(),
        });
    }

    if row.price <= 0 {
        return Err(ImportError {
            row_number,
            address: row.address.clone(),
            error: "price must be > 0".to_string(),
        });
    }

    let address = row.address.clone();
    let key_features =
        parse_json_array_field(&row.key_features, "key_features", row_number, &address)?;
    let neighborhood_highlights = parse_json_array_field(
        &row.neighborhood_highlights,
        "neighborhood_highlights",
        row_number,
        &address,
    )?;
    let nearby_amenities = parse_json_array_field(
        &row.nearby_amenities,
        "nearby_amenities",
        row_number,
        &address,
    )?;

    // Create property input
    let input = CreatePropertyInput {
        address: address.clone(),
        city: row.city,
        state: row.state,
        zip: row.zip,
        beds: row.beds,
        baths: row.baths,
        sqft: row.sqft,
        price: row.price,
        property_type: row.property_type,
        year_built: row.year_built,
        lot_size: row.lot_size,
        parking: row.parking,
        key_features,
        neighborhood: row.neighborhood,
        neighborhood_highlights,
        school_district: row.school_district,
        nearby_amenities,
        agent_notes: row.agent_notes,
    };

    // Create property in database
    properties::create(db, input)
        .await
        .map_err(|e| ImportError {
            row_number,
            address,
            error: format!("Database error: {}", e),
        })?;

    Ok(())
}

/// Generate a CSV template with headers and example data
pub fn generate_csv_template() -> String {
    let header = "address,city,state,zip,beds,baths,sqft,price,property_type,year_built,lot_size,parking,key_features,neighborhood,neighborhood_highlights,school_district,nearby_amenities,agent_notes";
    let example1 = r#"123 Oak Street,San Francisco,CA,94105,3,2.5,1800,95000000,single_family,2015,5000 sqft,2-car garage,"[""hardwood floors"",""pool"",""updated kitchen""]",Mission Bay,"[""walkable"",""near transit""]",SFUSD,"[""BART"",""Whole Foods"",""parks""]",Beautiful home in prime location"#;
    let example2 = r#"456 Pine Ave,Oakland,CA,94610,2,2,1200,75000000,condo,2018,,"1 parking spot","[""modern"",""city views""]",Downtown,"[""restaurants"",""nightlife""]",Oakland Unified,"[""Lake Merritt"",""BART""]","#;

    format!("{}\n{}\n{}\n", header, example1, example2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_pool;

    #[tokio::test]
    async fn test_import_valid_csv() {
        let db = test_pool().await;

        let csv_data = r#"address,city,state,zip,beds,baths,sqft,price,property_type,year_built,lot_size,parking,key_features,neighborhood,neighborhood_highlights,school_district,nearby_amenities,agent_notes
123 Main St,San Francisco,CA,94105,3,2.5,1800,95000000,single_family,2015,5000 sqft,2-car garage,"[]",Mission Bay,"[]",SFUSD,"[]",Test property
456 Oak Ave,Oakland,CA,94610,2,2,1200,75000000,condo,2018,,"1 spot","[]",Downtown,"[]",Oakland,"[]","#;

        let result = import_from_csv(&db, csv_data).await.unwrap();

        assert_eq!(result.total, 2);
        assert_eq!(result.successful, 2);
        assert_eq!(result.failed, 0);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_import_invalid_property_type() {
        let db = test_pool().await;

        let csv_data = r#"address,city,state,zip,beds,baths,sqft,price,property_type
123 Main St,SF,CA,94105,3,2.5,1800,95000000,invalid_type"#;

        let result = import_from_csv(&db, csv_data).await.unwrap();

        assert_eq!(result.total, 1);
        assert_eq!(result.successful, 0);
        assert_eq!(result.failed, 1);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].error.contains("Invalid property_type"));
    }

    #[tokio::test]
    async fn test_import_negative_beds() {
        let db = test_pool().await;

        let csv_data = r#"address,city,state,zip,beds,baths,sqft,price,property_type
123 Main St,SF,CA,94105,-1,2.5,1800,95000000,condo"#;

        let result = import_from_csv(&db, csv_data).await.unwrap();

        assert_eq!(result.failed, 1);
        assert!(result.errors[0].error.contains("beds must be >= 0"));
    }

    #[test]
    fn test_generate_csv_template() {
        let template = generate_csv_template();
        assert!(template.contains("address,city,state"));
        assert!(template.contains("123 Oak Street"));
        assert!(template.contains("456 Pine Ave"));
    }
}

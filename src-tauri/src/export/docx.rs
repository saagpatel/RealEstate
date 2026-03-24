use docx_rs::*;
use std::fs;

use crate::db::listings::Listing;
use crate::db::photos::Photo;
use crate::db::properties::Property;
use crate::error::AppError;
use crate::export::templates::ExportTemplate;

/// Generate a DOCX document for a property with its listings and photos
pub fn generate_docx(
    property: &Property,
    listings: &[Listing],
    photos: &[Photo],
    template: ExportTemplate,
) -> Result<Vec<u8>, AppError> {
    let config = template.config();
    let heading_1_size = usize::from(config.header_font_size) * 2;
    let heading_2_size = usize::from(config.header_font_size.saturating_sub(2)) * 2;
    let body_size = usize::from(config.body_font_size) * 2;
    let mut docx = Docx::new();

    // Property header
    let address = format!(
        "{}, {}, {} {}",
        property.address, property.city, property.state, property.zip
    );
    docx = docx.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(&address).bold().size(heading_1_size))
            .style("Heading1"),
    );

    // Property details
    let price = format_price_dollars(property.price);
    let details = format!(
        "${} | {} bed / {} bath / {} sqft | {}",
        price,
        property.beds,
        property.baths,
        property.sqft,
        property.property_type.replace('_', " "),
    );
    docx =
        docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&details).size(body_size)));

    if let Some(year) = property.year_built {
        docx = docx.add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Built: {}", year))
                    .size(body_size),
            ),
        );
    }

    // Key features
    let features: Vec<String> = serde_json::from_str(&property.key_features).unwrap_or_default();
    if !features.is_empty() {
        docx = docx.add_paragraph(Paragraph::new());
        docx = docx.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Key Features")
                        .bold()
                        .size(heading_2_size),
                )
                .style("Heading2"),
        );
        docx = docx.add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(features.join(" • ")).size(body_size)),
        );
    }

    // Photos section
    if config.include_photos && !photos.is_empty() {
        docx = docx.add_paragraph(Paragraph::new());
        docx = docx.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Property Photos")
                        .bold()
                        .size(heading_2_size),
                )
                .style("Heading2"),
        );

        // Add up to 6 photos
        for photo in photos.iter().take(6) {
            if let Ok(image_bytes) = fs::read(&photo.original_path) {
                // Create a Pic element with the image data
                const IMAGE_WIDTH_EMU: u32 = 4_000_000; // ~4.4 inches
                const IMAGE_HEIGHT_EMU: u32 = 3_000_000; // ~3.3 inches
                let pic = Pic::new(&image_bytes).size(IMAGE_WIDTH_EMU, IMAGE_HEIGHT_EMU);

                docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)));

                // Add caption if available
                if let Some(ref caption) = photo.caption {
                    if !caption.is_empty() {
                        docx = docx.add_paragraph(
                            Paragraph::new().add_run(
                                Run::new()
                                    .add_text(caption)
                                    .italic()
                                    .size(body_size.saturating_sub(2)),
                            ),
                        );
                    }
                }

                docx = docx.add_paragraph(Paragraph::new()); // Spacer
            } else {
                eprintln!("Failed to read image file: {}", photo.original_path);
                // Continue with other photos even if one fails
            }
        }
    }

    // Listings
    for (i, listing) in listings.iter().enumerate() {
        docx = docx.add_paragraph(Paragraph::new()); // spacer

        let section_title = match listing.generation_type.as_str() {
            "listing" => format!("Listing Description {}", i + 1),
            t if t.starts_with("social_") => {
                format!("Social Media - {}", t.strip_prefix("social_").unwrap_or(t))
            }
            t if t.starts_with("email_") => {
                format!("Email - {}", t.strip_prefix("email_").unwrap_or(t))
            }
            t => t.to_string(),
        };

        docx = docx.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&section_title)
                        .bold()
                        .size(heading_2_size),
                )
                .style("Heading2"),
        );

        // Split content by paragraphs
        for paragraph in listing.content.split("\n\n") {
            let trimmed = paragraph.trim();
            if !trimmed.is_empty() {
                docx = docx.add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(trimmed).size(body_size)),
                );
            }
        }
    }

    // Render to bytes
    let mut buf = Vec::new();
    docx.build()
        .pack(&mut std::io::Cursor::new(&mut buf))
        .map_err(|e| AppError::Export(format!("Failed to generate DOCX: {}", e)))?;

    Ok(buf)
}

fn format_price_dollars(cents: i64) -> String {
    let dollars = cents / 100;
    let mut s = dollars.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.drain(..).collect();
    for (i, c) in chars.iter().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, *c);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_property() -> Property {
        Property {
            id: "test".to_string(),
            address: "123 Oak St".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip: "94105".to_string(),
            beds: 3,
            baths: 2.5,
            sqft: 1800,
            price: 95000000,
            property_type: "single_family".to_string(),
            year_built: Some(2015),
            lot_size: None,
            parking: None,
            key_features: r#"["pool","hardwood floors"]"#.to_string(),
            neighborhood: None,
            neighborhood_highlights: "[]".to_string(),
            school_district: None,
            nearby_amenities: "[]".to_string(),
            agent_notes: None,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn sample_listing() -> Listing {
        Listing {
            id: "listing-1".to_string(),
            property_id: "test".to_string(),
            content: "A beautiful home in San Francisco.\n\nThis stunning property features hardwood floors and a pool.".to_string(),
            generation_type: "listing".to_string(),
            style: Some("luxury".to_string()),
            tone: Some("warm".to_string()),
            length: Some("medium".to_string()),
            seo_keywords: "[]".to_string(),
            brand_voice_id: None,
            tokens_used: 500,
            generation_cost_cents: 1,
            is_favorite: false,
            created_at: "2024-01-01".to_string(),
        }
    }

    fn missing_photo() -> Photo {
        Photo {
            id: "photo-1".to_string(),
            property_id: "test".to_string(),
            filename: "missing.jpg".to_string(),
            original_path: "/tmp/realestate-missing-photo.jpg".to_string(),
            thumbnail_path: "/tmp/realestate-missing-thumb.jpg".to_string(),
            sort_order: 0,
            caption: Some("Front exterior".to_string()),
            created_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_generate_docx_produces_valid_zip() {
        let property = sample_property();
        let listings = vec![sample_listing()];
        let result = generate_docx(&property, &listings, &[], ExportTemplate::Professional);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        // DOCX is a ZIP file — check magic bytes
        assert!(bytes.len() > 4);
        assert_eq!(&bytes[0..2], b"PK");
    }

    #[test]
    fn test_generate_docx_ignores_missing_photo_files() {
        let property = sample_property();
        let listings = vec![sample_listing()];
        let photos = vec![missing_photo()];

        let result = generate_docx(&property, &listings, &photos, ExportTemplate::Professional);

        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(bytes.len() > 4);
        assert_eq!(&bytes[0..2], b"PK");
    }
}

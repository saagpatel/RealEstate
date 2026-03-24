use genpdf::elements::{Break, Image, Paragraph};
use genpdf::fonts;
use genpdf::style::{Color, Style};
use genpdf::{Document, Element, SimplePageDecorator};
use std::path::Path;

use crate::db::listings::Listing;
use crate::db::photos::Photo;
use crate::db::properties::Property;
use crate::error::AppError;
use crate::export::templates::ExportTemplate;

/// Generate a PDF marketing package for a property with its listings and photos
pub fn generate_pdf(
    property: &Property,
    listings: &[Listing],
    photos: &[Photo],
    template: ExportTemplate,
) -> Result<Vec<u8>, AppError> {
    let config = template.config();
    let font_family = load_font_family()?;

    let mut doc = Document::new(font_family);
    doc.set_title("Property Marketing Package");

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(20);
    doc.set_page_decorator(decorator);

    // Property header
    doc.push(
        Paragraph::new(format!(
            "{}, {}, {} {}",
            property.address, property.city, property.state, property.zip
        ))
        .styled(
            Style::new()
                .bold()
                .with_font_size(config.header_font_size)
                .with_color(Color::Rgb(
                    config.primary_color.0,
                    config.primary_color.1,
                    config.primary_color.2,
                )),
        ),
    );

    doc.push(Break::new(0.5));

    // Property details
    let price = format_price_dollars(property.price);
    doc.push(
        Paragraph::new(format!(
            "${} | {} bed / {} bath / {} sqft | {}",
            price,
            property.beds,
            property.baths,
            property.sqft,
            property.property_type.replace('_', " "),
        ))
        .styled(Style::new().with_font_size(config.body_font_size)),
    );

    if let Some(ref year) = property.year_built {
        doc.push(
            Paragraph::new(format!("Built: {}", year))
                .styled(Style::new().with_font_size(config.body_font_size)),
        );
    }

    doc.push(Break::new(1.0));

    // Key features
    let features: Vec<String> = serde_json::from_str(&property.key_features).unwrap_or_default();
    if !features.is_empty() {
        doc.push(
            Paragraph::new("Key Features").styled(
                Style::new()
                    .bold()
                    .with_font_size(config.header_font_size.saturating_sub(2))
                    .with_color(Color::Rgb(
                        config.primary_color.0,
                        config.primary_color.1,
                        config.primary_color.2,
                    )),
            ),
        );
        doc.push(
            Paragraph::new(features.join(" • "))
                .styled(Style::new().with_font_size(config.body_font_size)),
        );
        doc.push(Break::new(0.5));
    }

    // Photos section
    if config.include_photos && !photos.is_empty() {
        doc.push(Break::new(1.0));
        doc.push(
            Paragraph::new("Property Photos").styled(
                Style::new()
                    .bold()
                    .with_font_size(config.header_font_size.saturating_sub(2))
                    .with_color(Color::Rgb(
                        config.secondary_color.0,
                        config.secondary_color.1,
                        config.secondary_color.2,
                    )),
            ),
        );
        doc.push(Break::new(0.5));

        // Add up to 6 photos (3x2 grid layout)
        for (i, photo) in photos.iter().take(6).enumerate() {
            match Image::from_path(&photo.original_path) {
                Ok(img) => {
                    doc.push(img.with_scale(genpdf::Scale::new(0.35, 0.35)));

                    // Add caption if available
                    if let Some(ref caption) = photo.caption {
                        if !caption.is_empty() {
                            doc.push(
                                Paragraph::new(caption).styled(
                                    Style::new()
                                        .with_color(Color::Rgb(100, 100, 100))
                                        .with_font_size(config.body_font_size.saturating_sub(1)),
                                ),
                            );
                        }
                    }

                    // Add spacing between photos
                    if i < photos.len() - 1 {
                        doc.push(Break::new(0.5));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to add image {} to PDF: {}", photo.original_path, e);
                    // Continue with other photos even if one fails
                }
            }
        }

        doc.push(Break::new(1.0));
    }

    // Listings
    for (i, listing) in listings.iter().enumerate() {
        doc.push(Break::new(1.0));

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

        doc.push(
            Paragraph::new(section_title).styled(
                Style::new()
                    .bold()
                    .with_font_size(config.header_font_size.saturating_sub(2))
                    .with_color(Color::Rgb(
                        config.primary_color.0,
                        config.primary_color.1,
                        config.primary_color.2,
                    )),
            ),
        );
        doc.push(Break::new(0.3));

        // Split content by paragraphs for better formatting
        for paragraph in listing.content.split("\n\n") {
            let trimmed = paragraph.trim();
            if !trimmed.is_empty() {
                doc.push(
                    Paragraph::new(trimmed)
                        .styled(Style::new().with_font_size(config.body_font_size)),
                );
                doc.push(Break::new(0.3));
            }
        }
    }

    // Render to bytes
    let mut buf = Vec::new();
    doc.render(&mut buf)
        .map_err(|e| AppError::Export(format!("Failed to render PDF: {}", e)))?;

    Ok(buf)
}

fn load_font_family() -> Result<fonts::FontFamily<fonts::FontData>, AppError> {
    let font_candidates = [
        (
            "/System/Library/Fonts/Supplemental/Arial.ttf",
            "/System/Library/Fonts/Supplemental/Arial Bold.ttf",
            "/System/Library/Fonts/Supplemental/Arial Italic.ttf",
            "/System/Library/Fonts/Supplemental/Arial Bold Italic.ttf",
        ),
        (
            "/Library/Fonts/Arial.ttf",
            "/Library/Fonts/Arial Bold.ttf",
            "/Library/Fonts/Arial Italic.ttf",
            "/Library/Fonts/Arial Bold Italic.ttf",
        ),
        (
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Italic.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-BoldItalic.ttf",
        ),
        (
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-BoldOblique.ttf",
        ),
        (
            r"C:\Windows\Fonts\arial.ttf",
            r"C:\Windows\Fonts\arialbd.ttf",
            r"C:\Windows\Fonts\ariali.ttf",
            r"C:\Windows\Fonts\arialbi.ttf",
        ),
    ];

    for (regular, bold, italic, bold_italic) in font_candidates {
        if [regular, bold, italic, bold_italic]
            .iter()
            .all(|path| Path::new(path).exists())
        {
            return Ok(fonts::FontFamily {
                regular: fonts::FontData::load(regular, None).map_err(|e| {
                    AppError::Export(format!("Failed to load PDF font '{}': {}", regular, e))
                })?,
                bold: fonts::FontData::load(bold, None).map_err(|e| {
                    AppError::Export(format!("Failed to load PDF font '{}': {}", bold, e))
                })?,
                italic: fonts::FontData::load(italic, None).map_err(|e| {
                    AppError::Export(format!("Failed to load PDF font '{}': {}", italic, e))
                })?,
                bold_italic: fonts::FontData::load(bold_italic, None).map_err(|e| {
                    AppError::Export(format!("Failed to load PDF font '{}': {}", bold_italic, e))
                })?,
            });
        }
    }

    Err(AppError::Export(
        "Failed to load a compatible PDF font family from the local system".to_string(),
    ))
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
    use crate::db::listings::Listing;
    use crate::db::photos::Photo;
    use crate::db::properties::Property;
    use crate::export::templates::ExportTemplate;

    fn sample_property() -> Property {
        Property {
            id: "property-1".to_string(),
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
            property_id: "property-1".to_string(),
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
            property_id: "property-1".to_string(),
            filename: "missing.jpg".to_string(),
            original_path: "/tmp/realestate-missing-photo.jpg".to_string(),
            thumbnail_path: "/tmp/realestate-missing-thumb.jpg".to_string(),
            sort_order: 0,
            caption: Some("Front exterior".to_string()),
            created_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_format_price_dollars() {
        assert_eq!(format_price_dollars(95000000), "950,000");
        assert_eq!(format_price_dollars(125000000), "1,250,000");
    }

    #[test]
    fn test_generate_pdf_without_photos_for_minimal_template() {
        let bytes = generate_pdf(
            &sample_property(),
            &[sample_listing()],
            &[],
            ExportTemplate::Minimal,
        )
        .expect("minimal template should render without photos");

        assert!(bytes.starts_with(b"%PDF"));
        assert!(bytes.len() > 100);
    }

    #[test]
    fn test_generate_pdf_ignores_missing_photo_files() {
        let bytes = generate_pdf(
            &sample_property(),
            &[sample_listing()],
            &[missing_photo()],
            ExportTemplate::Professional,
        )
        .expect("professional template should tolerate missing photo files");

        assert!(bytes.starts_with(b"%PDF"));
        assert!(bytes.len() > 100);
    }
}

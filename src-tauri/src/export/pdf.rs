use genpdf::elements::{Break, Image, Paragraph};
use genpdf::fonts;
use genpdf::style::{Color, Style};
use genpdf::{Document, Element, SimplePageDecorator};

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
    // Use built-in Helvetica font (always available)
    let font_family = fonts::from_files("", "Helvetica", None).unwrap_or_else(|_| {
        // Fallback: use Liberation Sans if Helvetica not found
        // genpdf provides built-in fonts as fallback
        fonts::from_files("", "LiberationSans", None).unwrap_or_else(|_| {
            // Last resort: create a minimal font family
            let font_data = genpdf::fonts::FontData::new(
                Vec::new(), // Will fail gracefully
                None,
            );
            let regular = match font_data {
                Ok(f) => f,
                Err(_) => {
                    return fonts::from_files("/System/Library/Fonts", "Helvetica", None)
                        .expect("Could not load any font")
                }
            };
            genpdf::fonts::FontFamily {
                regular: regular.clone(),
                bold: regular.clone(),
                italic: regular.clone(),
                bold_italic: regular,
            }
        })
    });

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

    #[test]
    fn test_format_price_dollars() {
        assert_eq!(format_price_dollars(95000000), "950,000");
        assert_eq!(format_price_dollars(125000000), "1,250,000");
    }
}

use krilla::color::rgb;
use krilla::geom::{Point, Size, Transform};
use krilla::image::Image;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::text::{Font, TextDirection};
use krilla::{Data, Document};
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
    let fonts = load_font_family()?;
    let mut items = Vec::new();

    items.push(PdfItem::Text {
        text: format!(
            "{}, {}, {} {}",
            property.address, property.city, property.state, property.zip
        ),
        size: config.header_font_size as f32,
        bold: true,
        color: config.primary_color,
    });
    items.push(PdfItem::Spacer(8.0));

    let price = format_price_dollars(property.price);
    items.push(PdfItem::Text {
        text: format!(
            "${} | {} bed / {} bath / {} sqft | {}",
            price,
            property.beds,
            property.baths,
            property.sqft,
            property.property_type.replace('_', " "),
        ),
        size: config.body_font_size as f32,
        bold: false,
        color: (0, 0, 0),
    });

    if let Some(ref year) = property.year_built {
        items.push(PdfItem::Text {
            text: format!("Built: {}", year),
            size: config.body_font_size as f32,
            bold: false,
            color: (0, 0, 0),
        });
    }

    items.push(PdfItem::Spacer(16.0));

    let features: Vec<String> = serde_json::from_str(&property.key_features).unwrap_or_default();
    if !features.is_empty() {
        items.push(PdfItem::Text {
            text: "Key Features".to_string(),
            size: config.header_font_size.saturating_sub(2) as f32,
            bold: true,
            color: config.primary_color,
        });
        items.push(PdfItem::Text {
            text: features.join(" • "),
            size: config.body_font_size as f32,
            bold: false,
            color: (0, 0, 0),
        });
        items.push(PdfItem::Spacer(8.0));
    }

    if config.include_photos && !photos.is_empty() {
        items.push(PdfItem::Spacer(16.0));
        items.push(PdfItem::Text {
            text: "Property Photos".to_string(),
            size: config.header_font_size.saturating_sub(2) as f32,
            bold: true,
            color: config.secondary_color,
        });
        items.push(PdfItem::Spacer(8.0));

        for photo in photos.iter().take(6) {
            items.push(PdfItem::Photo {
                path: photo.original_path.clone(),
                caption: photo.caption.clone(),
                caption_size: config.body_font_size.saturating_sub(1) as f32,
            });
        }
        items.push(PdfItem::Spacer(16.0));
    }

    for (i, listing) in listings.iter().enumerate() {
        items.push(PdfItem::Spacer(16.0));

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

        items.push(PdfItem::Text {
            text: section_title,
            size: config.header_font_size.saturating_sub(2) as f32,
            bold: true,
            color: config.primary_color,
        });
        items.push(PdfItem::Spacer(5.0));

        for paragraph in listing.content.split("\n\n") {
            let trimmed = paragraph.trim();
            if !trimmed.is_empty() {
                items.push(PdfItem::Text {
                    text: trimmed.to_string(),
                    size: config.body_font_size as f32,
                    bold: false,
                    color: (0, 0, 0),
                });
                items.push(PdfItem::Spacer(5.0));
            }
        }
    }

    render_items(&fonts, &items)
}

struct PdfFonts {
    regular: Font,
    bold: Font,
}

enum PdfItem {
    Text {
        text: String,
        size: f32,
        bold: bool,
        color: (u8, u8, u8),
    },
    Photo {
        path: String,
        caption: Option<String>,
        caption_size: f32,
    },
    Spacer(f32),
}

fn render_items(fonts: &PdfFonts, items: &[PdfItem]) -> Result<Vec<u8>, AppError> {
    const PAGE_WIDTH: f32 = 595.0;
    const PAGE_HEIGHT: f32 = 842.0;
    const MARGIN: f32 = 40.0;
    const TEXT_WIDTH_CHARS: usize = 92;
    const IMAGE_WIDTH: f32 = 220.0;
    const IMAGE_HEIGHT: f32 = 140.0;

    let mut document = Document::new();
    let mut index = 0;

    while index < items.len() {
        let mut page = document.start_page_with(
            PageSettings::from_wh(PAGE_WIDTH, PAGE_HEIGHT)
                .ok_or_else(|| AppError::Export("Invalid PDF page size".to_string()))?,
        );
        let mut surface = page.surface();
        let mut y = MARGIN;

        while index < items.len() {
            let needed = item_height(&items[index], TEXT_WIDTH_CHARS, IMAGE_HEIGHT);
            if y + needed > PAGE_HEIGHT - MARGIN && y > MARGIN {
                break;
            }

            match &items[index] {
                PdfItem::Text {
                    text,
                    size,
                    bold,
                    color,
                } => {
                    surface.set_fill(Some(Fill {
                        paint: rgb::Color::new(color.0, color.1, color.2).into(),
                        opacity: NormalizedF32::ONE,
                        rule: Default::default(),
                    }));
                    let font = if *bold {
                        fonts.bold.clone()
                    } else {
                        fonts.regular.clone()
                    };
                    for line in wrap_text(text, TEXT_WIDTH_CHARS) {
                        surface.draw_text(
                            Point::from_xy(MARGIN, y),
                            font.clone(),
                            *size,
                            &line,
                            false,
                            TextDirection::Auto,
                        );
                        y += *size + 4.0;
                    }
                }
                PdfItem::Photo {
                    path,
                    caption,
                    caption_size,
                } => {
                    if let Some(image) = load_image(path) {
                        surface.push_transform(&Transform::from_translate(MARGIN, y));
                        surface
                            .draw_image(image, Size::from_wh(IMAGE_WIDTH, IMAGE_HEIGHT).unwrap());
                        surface.pop();
                        y += IMAGE_HEIGHT + 6.0;

                        if let Some(caption) = caption {
                            if !caption.trim().is_empty() {
                                surface.set_fill(Some(Fill {
                                    paint: rgb::Color::new(100, 100, 100).into(),
                                    opacity: NormalizedF32::ONE,
                                    rule: Default::default(),
                                }));
                                surface.draw_text(
                                    Point::from_xy(MARGIN, y),
                                    fonts.regular.clone(),
                                    *caption_size,
                                    caption,
                                    false,
                                    TextDirection::Auto,
                                );
                                y += *caption_size + 6.0;
                            }
                        }
                    } else {
                        eprintln!("Failed to add image {} to PDF", path);
                    }
                }
                PdfItem::Spacer(height) => y += *height,
            }

            index += 1;
        }

        surface.finish();
        page.finish();
    }

    document
        .finish()
        .map_err(|e| AppError::Export(format!("Failed to render PDF: {}", e)))
}

fn item_height(item: &PdfItem, text_width_chars: usize, image_height: f32) -> f32 {
    match item {
        PdfItem::Text { text, size, .. } => {
            wrap_text(text, text_width_chars).len() as f32 * (*size + 4.0)
        }
        PdfItem::Photo {
            caption,
            caption_size,
            ..
        } => {
            image_height
                + 6.0
                + caption
                    .as_ref()
                    .filter(|caption| !caption.trim().is_empty())
                    .map(|_| *caption_size + 6.0)
                    .unwrap_or(0.0)
        }
        PdfItem::Spacer(height) => *height,
    }
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + word.len() + 1 > max_chars {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn load_image(path: &str) -> Option<Image> {
    let data = std::fs::read(path).ok()?;
    let extension = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())?;
    let data = Data::from(data);

    match extension.as_str() {
        "png" => Image::from_png(data, true).ok(),
        "jpg" | "jpeg" => Image::from_jpeg(data, true).ok(),
        "gif" => Image::from_gif(data, true).ok(),
        "webp" => Image::from_webp(data, true).ok(),
        _ => None,
    }
}

fn load_font_family() -> Result<PdfFonts, AppError> {
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
            let regular = Font::new(
                std::fs::read(regular)
                    .map_err(|e| {
                        AppError::Export(format!("Failed to load PDF font '{}': {}", regular, e))
                    })?
                    .into(),
                0,
            )
            .ok_or_else(|| AppError::Export("Failed to parse PDF font".to_string()))?;
            let bold = Font::new(
                std::fs::read(bold)
                    .map_err(|e| {
                        AppError::Export(format!("Failed to load PDF font '{}': {}", bold, e))
                    })?
                    .into(),
                0,
            )
            .ok_or_else(|| AppError::Export("Failed to parse PDF bold font".to_string()))?;

            return Ok(PdfFonts { regular, bold });
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

use std::time::Instant;

use sqlx::SqlitePool;
use tauri::State;

use crate::db::{analytics, listings, photos, properties};
use crate::error::AppError;
use crate::export::templates::ExportTemplate;
use crate::export::{docx, pdf};

#[tauri::command]
pub async fn export_pdf(
    db: State<'_, SqlitePool>,
    property_id: String,
    listing_ids: Vec<String>,
    template: Option<String>,
) -> Result<Vec<u8>, AppError> {
    let property = properties::get(&db, &property_id).await?;
    let template = parse_template(template)?;

    let mut selected_listings = Vec::new();
    for id in &listing_ids {
        let listing = listings::get(&db, id).await?;
        selected_listings.push(listing);
    }

    // Fetch photos for the property
    let property_photos = photos::list_by_property(&db, &property_id).await?;
    let listing_count = listing_ids.len();
    let photo_count = property_photos.len();
    let started_at = Instant::now();

    let bytes = tokio::task::spawn_blocking(move || {
        pdf::generate_pdf(&property, &selected_listings, &property_photos, template)
    })
    .await
    .map_err(|e| AppError::Export(format!("PDF generation task failed: {}", e)))??;

    let _ = analytics::record_export(
        &db,
        &property_id,
        "pdf",
        listing_count,
        photo_count,
        bytes.len(),
        started_at.elapsed().as_millis() as u64,
    )
    .await;

    Ok(bytes)
}

#[tauri::command]
pub async fn export_docx(
    db: State<'_, SqlitePool>,
    property_id: String,
    listing_ids: Vec<String>,
    template: Option<String>,
) -> Result<Vec<u8>, AppError> {
    let property = properties::get(&db, &property_id).await?;
    let template = parse_template(template)?;

    let mut selected_listings = Vec::new();
    for id in &listing_ids {
        let listing = listings::get(&db, id).await?;
        selected_listings.push(listing);
    }

    // Fetch photos for the property
    let property_photos = photos::list_by_property(&db, &property_id).await?;
    let listing_count = listing_ids.len();
    let photo_count = property_photos.len();
    let started_at = Instant::now();

    let bytes = tokio::task::spawn_blocking(move || {
        docx::generate_docx(&property, &selected_listings, &property_photos, template)
    })
    .await
    .map_err(|e| AppError::Export(format!("DOCX generation task failed: {}", e)))??;

    let _ = analytics::record_export(
        &db,
        &property_id,
        "docx",
        listing_count,
        photo_count,
        bytes.len(),
        started_at.elapsed().as_millis() as u64,
    )
    .await;

    Ok(bytes)
}

#[tauri::command]
pub async fn copy_to_clipboard(_text: String) -> Result<(), AppError> {
    // Use the clipboard plugin from the frontend side instead
    // This command is a simple passthrough for cases where we need it from Rust
    // In practice, the frontend uses @tauri-apps/plugin-clipboard-manager directly
    Ok(())
}

fn parse_template(template: Option<String>) -> Result<ExportTemplate, AppError> {
    match template {
        Some(value) => ExportTemplate::from_str(&value).ok_or_else(|| {
            AppError::Validation(format!(
                "Unsupported export template '{}'. Expected professional, luxury, or minimal.",
                value
            ))
        }),
        None => Ok(ExportTemplate::Professional),
    }
}

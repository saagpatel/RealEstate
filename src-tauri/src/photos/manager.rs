use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoRecord {
    pub id: String,
    pub property_id: String,
    pub filename: String,
    pub original_path: String,
    pub thumbnail_path: String,
    pub sort_order: i64,
}

const THUMB_WIDTH: u32 = 300;
const THUMB_HEIGHT: u32 = 200;
const THUMB_QUALITY: u8 = 85;

pub fn import_photos(
    app_data_dir: &Path,
    property_id: &str,
    source_paths: &[PathBuf],
) -> Result<Vec<PhotoRecord>, AppError> {
    let photos_dir = app_data_dir.join("photos").join(property_id);
    let thumbs_dir = photos_dir.join("thumbs");

    std::fs::create_dir_all(&photos_dir)
        .map_err(|e| AppError::Photo(format!("Failed to create photos directory: {}", e)))?;
    std::fs::create_dir_all(&thumbs_dir)
        .map_err(|e| AppError::Photo(format!("Failed to create thumbnails directory: {}", e)))?;

    let mut records = Vec::new();

    for (index, source) in source_paths.iter().enumerate() {
        let ext = source
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg")
            .to_lowercase();

        let filename = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("photo")
            .to_string();

        let id = uuid::Uuid::new_v4().to_string();
        let dest_filename = format!("{}.{}", id, ext);
        let dest_path = photos_dir.join(&dest_filename);
        let thumb_path = thumbs_dir.join(format!("{}.jpg", id));

        // Copy original file
        std::fs::copy(source, &dest_path)
            .map_err(|e| AppError::Photo(format!("Failed to copy photo {}: {}", filename, e)))?;

        // Generate thumbnail
        create_thumbnail(&dest_path, &thumb_path).map_err(|e| {
            // Clean up the copied original if thumbnail fails
            let _ = std::fs::remove_file(&dest_path);
            AppError::Photo(format!(
                "Failed to create thumbnail for {}: {}",
                filename, e
            ))
        })?;

        records.push(PhotoRecord {
            id,
            property_id: property_id.to_string(),
            filename,
            original_path: dest_path.to_string_lossy().to_string(),
            thumbnail_path: thumb_path.to_string_lossy().to_string(),
            sort_order: index as i64,
        });
    }

    Ok(records)
}

fn create_thumbnail(source: &Path, dest: &Path) -> Result<(), AppError> {
    let img =
        image::open(source).map_err(|e| AppError::Photo(format!("Failed to open image: {}", e)))?;

    let thumb = img.resize_to_fill(THUMB_WIDTH, THUMB_HEIGHT, FilterType::Lanczos3);

    // Encode as JPEG with specified quality
    let mut output = std::io::BufWriter::new(
        std::fs::File::create(dest)
            .map_err(|e| AppError::Photo(format!("Failed to create thumbnail file: {}", e)))?,
    );

    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, THUMB_QUALITY);
    thumb
        .write_with_encoder(encoder)
        .map_err(|e| AppError::Photo(format!("Failed to encode thumbnail: {}", e)))?;

    Ok(())
}

pub fn delete_photo_files(original_path: &str, thumbnail_path: &str) -> Result<(), AppError> {
    if Path::new(original_path).exists() {
        std::fs::remove_file(original_path)
            .map_err(|e| AppError::Photo(format!("Failed to delete original photo: {}", e)))?;
    }

    if Path::new(thumbnail_path).exists() {
        std::fs::remove_file(thumbnail_path)
            .map_err(|e| AppError::Photo(format!("Failed to delete thumbnail: {}", e)))?;
    }

    Ok(())
}

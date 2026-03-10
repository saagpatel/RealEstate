use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::db::photos::{self, Photo};
use crate::error::AppError;
use crate::photos::manager;

#[tauri::command]
pub async fn import_photos(
    app_handle: AppHandle,
    pool: State<'_, SqlitePool>,
    property_id: String,
) -> Result<Vec<Photo>, AppError> {
    // Open file dialog on a blocking thread (dialog must not run on main thread)
    let file_paths = {
        let handle = app_handle.clone();
        tauri::async_runtime::spawn_blocking(move || {
            handle
                .dialog()
                .file()
                .add_filter(
                    "Images",
                    &["jpg", "jpeg", "png", "gif", "webp", "bmp", "tiff"],
                )
                .set_title("Select Property Photos")
                .blocking_pick_files()
        })
        .await
        .map_err(|e| AppError::Photo(format!("Dialog thread error: {}", e)))?
    };

    let file_paths = match file_paths {
        Some(paths) => paths,
        None => return Ok(Vec::new()), // User cancelled
    };

    // Convert FilePaths to PathBufs
    let source_paths: Vec<std::path::PathBuf> = file_paths
        .into_iter()
        .filter_map(|fp| fp.into_path().ok())
        .collect();

    if source_paths.is_empty() {
        return Ok(Vec::new());
    }

    // Check existing photo count
    let existing = photos::list_by_property(&pool, &property_id).await?;
    let remaining_slots = 20_usize.saturating_sub(existing.len());
    if remaining_slots == 0 {
        return Err(AppError::Validation(
            "Maximum of 20 photos reached for this property".to_string(),
        ));
    }

    let paths_to_import = if source_paths.len() > remaining_slots {
        source_paths[..remaining_slots].to_vec()
    } else {
        source_paths
    };

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Photo(format!("Failed to get app data dir: {}", e)))?;

    // Determine starting sort order
    let start_order = existing.len() as i64;

    // Run file I/O (copy + thumbnail) on blocking thread
    let records = {
        let pid = property_id.clone();
        let paths = paths_to_import;
        tauri::async_runtime::spawn_blocking(move || {
            manager::import_photos(&app_data_dir, &pid, &paths)
        })
        .await
        .map_err(|e| AppError::Photo(format!("Import thread error: {}", e)))??
    };

    // Insert into DB
    let mut inserted = Vec::new();
    for record in records {
        let photo = photos::insert(
            &pool,
            &record.id,
            &record.property_id,
            &record.filename,
            &record.original_path,
            &record.thumbnail_path,
            start_order + record.sort_order,
        )
        .await?;
        inserted.push(photo);
    }

    Ok(inserted)
}

#[tauri::command]
pub async fn list_photos(
    pool: State<'_, SqlitePool>,
    property_id: String,
) -> Result<Vec<Photo>, AppError> {
    photos::list_by_property(&pool, &property_id).await
}

#[tauri::command]
pub async fn delete_photo(
    app_handle: AppHandle,
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), AppError> {
    let photo = photos::get(&pool, &id).await?;

    // Delete files from disk
    let original = photo.original_path.clone();
    let thumb = photo.thumbnail_path.clone();
    let _handle = app_handle; // keep alive for lifetime

    tauri::async_runtime::spawn_blocking(move || manager::delete_photo_files(&original, &thumb))
        .await
        .map_err(|e| AppError::Photo(format!("Delete thread error: {}", e)))??;

    // Delete from DB
    photos::delete(&pool, &id).await
}

#[tauri::command]
pub async fn reorder_photos(
    pool: State<'_, SqlitePool>,
    property_id: String,
    photo_ids: Vec<String>,
) -> Result<(), AppError> {
    // Verify all photos belong to this property
    let existing = photos::list_by_property(&pool, &property_id).await?;
    let existing_ids: std::collections::HashSet<&str> =
        existing.iter().map(|p| p.id.as_str()).collect();

    for id in &photo_ids {
        if !existing_ids.contains(id.as_str()) {
            return Err(AppError::Validation(format!(
                "Photo {} does not belong to property {}",
                id, property_id
            )));
        }
    }

    // Update sort order based on position in the array
    for (index, photo_id) in photo_ids.iter().enumerate() {
        photos::update_sort_order(&pool, photo_id, index as i64).await?;
    }

    Ok(())
}

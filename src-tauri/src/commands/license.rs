use sqlx::SqlitePool;
use tauri::State;

use crate::db::settings;
use crate::error::AppError;
use crate::license::{self, LicenseStatus};

#[tauri::command]
pub async fn validate_license_key(
    db: State<'_, SqlitePool>,
    license_key: String,
) -> Result<LicenseStatus, AppError> {
    let status = license::validate_license(&license_key).await?;

    if status.is_valid {
        // Store the license key and validation timestamp
        settings::set(&db, "license_key", &license_key).await?;
        settings::set(&db, "license_validated_at", &status.validated_at).await?;
    }

    Ok(status)
}

#[tauri::command]
pub async fn check_license(db: State<'_, SqlitePool>) -> Result<LicenseStatus, AppError> {
    let license_key = settings::get(&db, "license_key").await.unwrap_or_default();

    if license_key.is_empty() {
        return Ok(LicenseStatus {
            is_valid: false,
            license_key: String::new(),
            validated_at: String::new(),
            error: Some("No license key found".to_string()),
        });
    }

    // Check if cached validation is still fresh
    let validated_at = settings::get(&db, "license_validated_at")
        .await
        .unwrap_or_default();

    if license::is_cache_valid(&validated_at) {
        return Ok(LicenseStatus {
            is_valid: true,
            license_key,
            validated_at,
            error: None,
        });
    }

    // Re-validate
    let status = license::validate_license(&license_key).await?;

    if status.is_valid {
        settings::set(&db, "license_validated_at", &status.validated_at).await?;
    }

    Ok(status)
}

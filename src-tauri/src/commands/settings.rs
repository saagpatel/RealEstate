use sqlx::SqlitePool;
use tauri::State;

use crate::db::settings;
use crate::error::AppError;

#[tauri::command]
pub async fn get_setting(pool: State<'_, SqlitePool>, key: String) -> Result<String, AppError> {
    settings::get(&pool, &key).await
}

#[tauri::command]
pub async fn set_setting(
    pool: State<'_, SqlitePool>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    settings::set(&pool, &key, &value).await
}

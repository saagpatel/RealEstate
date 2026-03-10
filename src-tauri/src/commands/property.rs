use sqlx::SqlitePool;
use tauri::State;

use crate::db::properties::{self, CreatePropertyInput, Property};
use crate::error::AppError;

#[tauri::command]
pub async fn create_property(
    pool: State<'_, SqlitePool>,
    input: CreatePropertyInput,
) -> Result<Property, AppError> {
    properties::create(&pool, input).await
}

#[tauri::command]
pub async fn get_property(pool: State<'_, SqlitePool>, id: String) -> Result<Property, AppError> {
    properties::get(&pool, &id).await
}

#[tauri::command]
pub async fn list_properties(pool: State<'_, SqlitePool>) -> Result<Vec<Property>, AppError> {
    properties::list_all(&pool).await
}

#[tauri::command]
pub async fn update_property(
    pool: State<'_, SqlitePool>,
    id: String,
    input: CreatePropertyInput,
) -> Result<Property, AppError> {
    properties::update(&pool, &id, input).await
}

#[tauri::command]
pub async fn delete_property(pool: State<'_, SqlitePool>, id: String) -> Result<(), AppError> {
    properties::delete(&pool, &id).await
}

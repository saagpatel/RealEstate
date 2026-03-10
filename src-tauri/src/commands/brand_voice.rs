use sqlx::SqlitePool;
use tauri::State;

use crate::ai::brand_voice::extract_voice;
use crate::ai::client::ClaudeClient;
use crate::db::{brand_voice, settings};
use crate::error::AppError;

#[tauri::command]
pub async fn create_brand_voice(
    db: State<'_, SqlitePool>,
    name: String,
    description: Option<String>,
    sample_listings: Vec<String>,
) -> Result<brand_voice::BrandVoice, AppError> {
    // Load API key
    let api_key = settings::get(&db, "api_key").await?;
    if api_key.is_empty() {
        return Err(AppError::Config(
            "No API key configured. Add your Anthropic API key in Settings.".to_string(),
        ));
    }

    // Load AI model preference
    let model = settings::get(&db, "ai_model")
        .await
        .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

    let client = ClaudeClient::new(api_key, model);
    let extracted_style = extract_voice(&client, &sample_listings).await?;

    let voice = brand_voice::create(
        &db,
        &name,
        description.as_deref(),
        &extracted_style,
        &sample_listings,
    )
    .await?;

    Ok(voice)
}

#[tauri::command]
pub async fn list_brand_voices(
    db: State<'_, SqlitePool>,
) -> Result<Vec<brand_voice::BrandVoice>, AppError> {
    brand_voice::list_all(&db).await
}

#[tauri::command]
pub async fn delete_brand_voice(db: State<'_, SqlitePool>, id: String) -> Result<(), AppError> {
    brand_voice::delete(&db, &id).await
}

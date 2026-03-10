use std::time::Instant;

use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::ipc::Channel;
use tauri::State;

use crate::ai::client::{ClaudeClient, StreamEvent};
use crate::ai::email_generator;
use crate::ai::listing_generator;
use crate::ai::prompts::{AgentInfo, GenerationOptions};
use crate::ai::social_generator;
use crate::db::analytics::GenerationAnalyticsRecord;
use crate::db::{analytics, brand_voice, listings, properties, settings};
use crate::error::AppError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateListingArgs {
    pub property_id: String,
    pub style: String,
    pub tone: String,
    pub length: String,
    pub seo_keywords: Vec<String>,
    pub brand_voice_id: Option<String>,
}

#[tauri::command]
pub async fn generate_listing(
    db: State<'_, SqlitePool>,
    args: GenerateListingArgs,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    // Load property
    let property = properties::get(&db, &args.property_id)
        .await
        .map_err(|_| AppError::PropertyNotFound(args.property_id.clone()))?;

    // Load API key
    let api_key = settings::get(&db, "api_key").await.ok();
    if api_key.is_none() || api_key.as_ref().unwrap().is_empty() {
        return Err(AppError::MissingApiKey);
    }
    let api_key = api_key.unwrap();

    // Load agent info
    let agent_info = AgentInfo {
        name: settings::get(&db, "agent_name").await.unwrap_or_default(),
        phone: settings::get(&db, "agent_phone").await.unwrap_or_default(),
        email: settings::get(&db, "agent_email").await.unwrap_or_default(),
        brokerage: settings::get(&db, "brokerage_name")
            .await
            .unwrap_or_default(),
    };

    // Load brand voice if specified
    let voice_block = if let Some(ref voice_id) = args.brand_voice_id {
        let voice = brand_voice::get(&db, voice_id).await?;
        crate::ai::prompts::build_voice_block(&voice.extracted_style)
    } else {
        None
    };

    let options = GenerationOptions {
        style: args.style.clone(),
        tone: args.tone.clone(),
        length: args.length.clone(),
        seo_keywords: args.seo_keywords.clone(),
    };

    // Load AI model preference
    let model = settings::get(&db, "ai_model")
        .await
        .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

    let client = ClaudeClient::new(api_key, model.clone());
    let started_at = Instant::now();

    let result = listing_generator::generate_listing(
        &client,
        &property,
        &options,
        voice_block.as_deref(),
        &agent_info,
        &on_event,
    )
    .await;

    let latency_ms = started_at.elapsed().as_millis() as u64;
    let result = match result {
        Ok(result) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: "listing",
                    model_used: &model,
                    input_tokens: result.input_tokens,
                    output_tokens: result.output_tokens,
                    cost_cents: result.cost_cents,
                    latency_ms,
                    success: true,
                    error_message: None,
                },
            )
            .await;
            result
        }
        Err(err) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: "listing",
                    model_used: &model,
                    input_tokens: 0,
                    output_tokens: 0,
                    cost_cents: 0,
                    latency_ms,
                    success: false,
                    error_message: Some(&err.to_string()),
                },
            )
            .await;
            return Err(err);
        }
    };

    // Save to database
    listings::save(
        &db,
        listings::CreateListingInput {
            property_id: args.property_id,
            content: result.full_text,
            generation_type: "listing".to_string(),
            style: Some(args.style),
            tone: Some(args.tone),
            length: Some(args.length),
            seo_keywords: args.seo_keywords,
            brand_voice_id: args.brand_voice_id,
            tokens_used: (result.input_tokens + result.output_tokens) as i64,
            generation_cost_cents: result.cost_cents as i64,
        },
    )
    .await?;

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSocialArgs {
    pub property_id: String,
    pub platform: String,
    pub brand_voice_id: Option<String>,
}

#[tauri::command]
pub async fn generate_social(
    db: State<'_, SqlitePool>,
    args: GenerateSocialArgs,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    // Load property
    let property = properties::get(&db, &args.property_id)
        .await
        .map_err(|_| AppError::PropertyNotFound(args.property_id.clone()))?;

    // Load API key
    let api_key = settings::get(&db, "api_key").await.ok();
    if api_key.is_none() || api_key.as_ref().unwrap().is_empty() {
        return Err(AppError::MissingApiKey);
    }
    let api_key = api_key.unwrap();

    // Load agent info
    let agent_info = AgentInfo {
        name: settings::get(&db, "agent_name").await.unwrap_or_default(),
        phone: settings::get(&db, "agent_phone").await.unwrap_or_default(),
        email: settings::get(&db, "agent_email").await.unwrap_or_default(),
        brokerage: settings::get(&db, "brokerage_name")
            .await
            .unwrap_or_default(),
    };

    // Load brand voice if specified
    let voice_block = if let Some(ref voice_id) = args.brand_voice_id {
        let voice = brand_voice::get(&db, voice_id).await?;
        crate::ai::prompts::build_voice_block(&voice.extracted_style)
    } else {
        None
    };

    // Load AI model preference
    let model = settings::get(&db, "ai_model")
        .await
        .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

    let client = ClaudeClient::new(api_key, model.clone());
    let started_at = Instant::now();

    let result = social_generator::generate_social_posts(
        &client,
        &property,
        &args.platform,
        voice_block.as_deref(),
        &agent_info,
        &on_event,
    )
    .await;

    let generation_type = format!("social_{}", args.platform);
    let latency_ms = started_at.elapsed().as_millis() as u64;
    let result = match result {
        Ok(result) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: &generation_type,
                    model_used: &model,
                    input_tokens: result.input_tokens,
                    output_tokens: result.output_tokens,
                    cost_cents: result.cost_cents,
                    latency_ms,
                    success: true,
                    error_message: None,
                },
            )
            .await;
            result
        }
        Err(err) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: &generation_type,
                    model_used: &model,
                    input_tokens: 0,
                    output_tokens: 0,
                    cost_cents: 0,
                    latency_ms,
                    success: false,
                    error_message: Some(&err.to_string()),
                },
            )
            .await;
            return Err(err);
        }
    };

    // Save to database
    listings::save(
        &db,
        listings::CreateListingInput {
            property_id: args.property_id,
            content: result.full_text,
            generation_type,
            style: None,
            tone: None,
            length: None,
            seo_keywords: vec![],
            brand_voice_id: args.brand_voice_id,
            tokens_used: (result.input_tokens + result.output_tokens) as i64,
            generation_cost_cents: result.cost_cents as i64,
        },
    )
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn list_listings(
    db: State<'_, SqlitePool>,
    property_id: String,
) -> Result<Vec<listings::Listing>, AppError> {
    listings::list_by_property(&db, &property_id).await
}

#[tauri::command]
pub async fn toggle_listing_favorite(
    db: State<'_, SqlitePool>,
    id: String,
) -> Result<(), AppError> {
    listings::toggle_favorite(&db, &id).await
}

#[tauri::command]
pub async fn delete_listing(db: State<'_, SqlitePool>, id: String) -> Result<(), AppError> {
    listings::delete(&db, &id).await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateEmailArgs {
    pub property_id: String,
    pub template_type: String,
    pub brand_voice_id: Option<String>,
}

#[tauri::command]
pub async fn generate_email(
    db: State<'_, SqlitePool>,
    args: GenerateEmailArgs,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    // Load property
    let property = properties::get(&db, &args.property_id)
        .await
        .map_err(|_| AppError::PropertyNotFound(args.property_id.clone()))?;

    // Load API key
    let api_key = settings::get(&db, "api_key").await.ok();
    if api_key.is_none() || api_key.as_ref().unwrap().is_empty() {
        return Err(AppError::MissingApiKey);
    }
    let api_key = api_key.unwrap();

    // Load agent info
    let agent_info = AgentInfo {
        name: settings::get(&db, "agent_name").await.unwrap_or_default(),
        phone: settings::get(&db, "agent_phone").await.unwrap_or_default(),
        email: settings::get(&db, "agent_email").await.unwrap_or_default(),
        brokerage: settings::get(&db, "brokerage_name")
            .await
            .unwrap_or_default(),
    };

    // Load brand voice if specified
    let voice_block = if let Some(ref voice_id) = args.brand_voice_id {
        let voice = brand_voice::get(&db, voice_id).await?;
        crate::ai::prompts::build_voice_block(&voice.extracted_style)
    } else {
        None
    };

    // Load AI model preference
    let model = settings::get(&db, "ai_model")
        .await
        .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

    let client = ClaudeClient::new(api_key, model.clone());
    let started_at = Instant::now();

    let result = email_generator::generate_email(
        &client,
        &property,
        &args.template_type,
        voice_block.as_deref(),
        &agent_info,
        &on_event,
    )
    .await;

    // Save to database with generation_type = "email_{template_type}"
    let generation_type = format!("email_{}", args.template_type);
    let latency_ms = started_at.elapsed().as_millis() as u64;
    let result = match result {
        Ok(result) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: &generation_type,
                    model_used: &model,
                    input_tokens: result.input_tokens,
                    output_tokens: result.output_tokens,
                    cost_cents: result.cost_cents,
                    latency_ms,
                    success: true,
                    error_message: None,
                },
            )
            .await;
            result
        }
        Err(err) => {
            let _ = analytics::record_generation(
                &db,
                GenerationAnalyticsRecord {
                    property_id: &property.id,
                    generation_type: &generation_type,
                    model_used: &model,
                    input_tokens: 0,
                    output_tokens: 0,
                    cost_cents: 0,
                    latency_ms,
                    success: false,
                    error_message: Some(&err.to_string()),
                },
            )
            .await;
            return Err(err);
        }
    };

    listings::save(
        &db,
        listings::CreateListingInput {
            property_id: args.property_id,
            content: result.full_text,
            generation_type,
            style: None,
            tone: None,
            length: None,
            seo_keywords: vec![],
            brand_voice_id: args.brand_voice_id,
            tokens_used: (result.input_tokens + result.output_tokens) as i64,
            generation_cost_cents: result.cost_cents as i64,
        },
    )
    .await?;

    Ok(())
}

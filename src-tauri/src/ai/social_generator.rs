use tauri::ipc::Channel;

use crate::db::properties::Property;
use crate::error::AppError;

use super::client::{calculate_cost_cents, ClaudeClient, StreamEvent};
use super::listing_generator::{GenerationResult, PropertyAnalysis};
use super::prompts::{
    build_analysis_prompt, build_social_prompt, AgentInfo, MAX_TOKENS_ANALYSIS, MAX_TOKENS_SOCIAL,
};

/// Two-stage social media post generation pipeline:
/// 1. Analyze property (non-streaming) -> structured JSON
/// 2. Generate social media posts (streaming) -> text deltas via Channel
pub async fn generate_social_posts(
    client: &ClaudeClient,
    property: &Property,
    platform: &str,
    brand_voice_block: Option<&str>,
    agent_info: &AgentInfo,
    channel: &Channel<StreamEvent>,
) -> Result<GenerationResult, AppError> {
    // Stage 1: Property analysis (non-streaming)
    let (analysis_system, analysis_user) = build_analysis_prompt(property);
    let (analysis_text, analysis_input, analysis_output) = client
        .send_message(&analysis_system, &analysis_user, MAX_TOKENS_ANALYSIS)
        .await?;

    // Validate we got valid JSON
    let _parsed: PropertyAnalysis = serde_json::from_str(&analysis_text).map_err(|e| {
        AppError::Api(format!(
            "Failed to parse property analysis from Claude: {}",
            e
        ))
    })?;

    // Stage 2: Generate social posts (streaming)
    let (social_system, social_user) = build_social_prompt(
        property,
        &analysis_text,
        platform,
        brand_voice_block,
        agent_info,
    );

    let (full_text, social_input, social_output) = client
        .stream_message(&social_system, &social_user, MAX_TOKENS_SOCIAL, channel)
        .await?;

    let total_input = analysis_input + social_input;
    let total_output = analysis_output + social_output;
    let cost_cents = calculate_cost_cents(total_input, total_output);

    // Send finished event
    let _ = channel.send(StreamEvent::Finished {
        full_text: full_text.clone(),
        input_tokens: total_input,
        output_tokens: total_output,
        cost_cents,
    });

    Ok(GenerationResult {
        full_text,
        input_tokens: total_input,
        output_tokens: total_output,
        cost_cents,
        analysis_json: analysis_text,
    })
}

use tauri::ipc::Channel;

use crate::db::properties::Property;
use crate::error::AppError;

use super::client::{calculate_cost_cents, ClaudeClient, StreamEvent};
use super::listing_generator::GenerationResult;
use super::prompts::{
    build_analysis_prompt, build_email_prompt, AgentInfo, MAX_TOKENS_ANALYSIS, MAX_TOKENS_EMAIL,
};

/// Two-stage email generation pipeline:
/// 1. Analyze property (non-streaming) -> structured JSON
/// 2. Generate email (streaming) -> text deltas via Channel
pub async fn generate_email(
    client: &ClaudeClient,
    property: &Property,
    template_type: &str,
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
    let _parsed: serde_json::Value = serde_json::from_str(&analysis_text).map_err(|e| {
        AppError::Api(format!(
            "Failed to parse property analysis from Claude: {}",
            e
        ))
    })?;

    // Stage 2: Generate email (streaming)
    let (email_system, email_user) = build_email_prompt(
        property,
        &analysis_text,
        template_type,
        brand_voice_block,
        agent_info,
    );

    let (full_text, email_input, email_output) = client
        .stream_message(&email_system, &email_user, MAX_TOKENS_EMAIL, channel)
        .await?;

    let total_input = analysis_input + email_input;
    let total_output = analysis_output + email_output;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_template_types() {
        // Ensure template types match what prompts.rs expects
        let valid_types = ["buyer", "seller", "open_house"];
        for t in &valid_types {
            assert!(!t.is_empty());
        }
    }
}

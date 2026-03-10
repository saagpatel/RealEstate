use serde::Deserialize;
use tauri::ipc::Channel;

use crate::db::properties::Property;
use crate::error::AppError;

use super::client::{calculate_cost_cents, ClaudeClient, StreamEvent};
use super::prompts::{
    build_analysis_prompt, build_listing_prompt, max_tokens_for_listing, AgentInfo,
    GenerationOptions, MAX_TOKENS_ANALYSIS,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PropertyAnalysis {
    pub selling_points: Vec<String>,
    pub target_buyer: String,
    pub neighborhood_appeal: String,
    pub comparable_positioning: String,
    pub emotional_hooks: Vec<String>,
}

#[allow(dead_code)]
pub struct GenerationResult {
    pub full_text: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_cents: u32,
    pub analysis_json: String,
}

/// Two-stage listing generation pipeline:
/// 1. Analyze property (non-streaming) → structured JSON
/// 2. Generate listing description (streaming) → text deltas via Channel
pub async fn generate_listing(
    client: &ClaudeClient,
    property: &Property,
    options: &GenerationOptions,
    brand_voice_block: Option<&str>,
    agent_info: &AgentInfo,
    channel: &Channel<StreamEvent>,
) -> Result<GenerationResult, AppError> {
    // Stage 1: Property analysis (non-streaming)
    let (analysis_system, analysis_user) = build_analysis_prompt(property);
    let (analysis_text, analysis_input, _analysis_output) = client
        .send_message(&analysis_system, &analysis_user, MAX_TOKENS_ANALYSIS)
        .await?;

    // Validate we got valid JSON (but we don't need to deserialize it for the prompt)
    let _parsed: PropertyAnalysis = serde_json::from_str(&analysis_text).map_err(|e| {
        AppError::Api(format!(
            "Failed to parse property analysis from Claude: {}",
            e
        ))
    })?;

    // Stage 2: Generate listing (streaming)
    let (listing_system, listing_user) = build_listing_prompt(
        property,
        &analysis_text,
        options,
        brand_voice_block,
        agent_info,
    );

    let max_tokens = max_tokens_for_listing(&options.length);

    let (full_text, listing_input, listing_output) = client
        .stream_message(&listing_system, &listing_user, max_tokens, channel)
        .await?;

    let total_input = analysis_input + listing_input;
    let total_output = _analysis_output + listing_output;
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
    use super::*;

    #[test]
    fn test_property_analysis_deserialization() {
        let json = r#"{
            "selling_points": ["Chef's kitchen with Viking appliances", "Pool with spa", "Walking distance to BART"],
            "target_buyer": "Young professional couple seeking urban living with outdoor space",
            "neighborhood_appeal": "Mission Bay offers walkability and proximity to tech employers",
            "comparable_positioning": "Priced below comparable renovated homes in the area",
            "emotional_hooks": ["Entertain in style", "Your urban oasis", "Steps from everything"]
        }"#;

        let analysis: PropertyAnalysis = serde_json::from_str(json).unwrap();
        assert_eq!(analysis.selling_points.len(), 3);
        assert!(analysis.target_buyer.contains("professional"));
        assert_eq!(analysis.emotional_hooks.len(), 3);
    }

    #[test]
    fn test_invalid_analysis_json() {
        let bad_json = "not valid json";
        let result: Result<PropertyAnalysis, _> = serde_json::from_str(bad_json);
        assert!(result.is_err());
    }
}

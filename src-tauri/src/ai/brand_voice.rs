use crate::ai::client::ClaudeClient;
use crate::ai::prompts::{build_voice_extraction_prompt, MAX_TOKENS_BRAND_VOICE};
use crate::error::AppError;

/// Extract brand voice style from sample listing descriptions
pub async fn extract_voice(
    client: &ClaudeClient,
    sample_listings: &[String],
) -> Result<String, AppError> {
    if sample_listings.len() < 2 {
        return Err(AppError::Validation(
            "At least 2 sample listings are required to extract a voice profile.".to_string(),
        ));
    }

    let (system, user) = build_voice_extraction_prompt(sample_listings);

    let (response, _input_tokens, _output_tokens) = client
        .send_message(&system, &user, MAX_TOKENS_BRAND_VOICE)
        .await?;

    // Validate the response is valid JSON
    let trimmed = response.trim();
    // Strip markdown code fences if Claude wraps in ```json
    let json_str = if trimmed.starts_with("```") {
        let inner = trimmed
            .strip_prefix("```json")
            .or_else(|| trimmed.strip_prefix("```"))
            .unwrap_or(trimmed);
        inner.strip_suffix("```").unwrap_or(inner).trim()
    } else {
        trimmed
    };

    // Validate it parses as JSON
    let _: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
        AppError::Api(format!(
            "Failed to parse brand voice extraction as JSON: {}",
            e
        ))
    })?;

    Ok(json_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_requires_two_samples() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = ClaudeClient::new(
            "fake-key".to_string(),
            "claude-sonnet-4-20250514".to_string(),
        );
        let result = rt.block_on(extract_voice(&client, &["single".to_string()]));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("At least 2"));
    }
}

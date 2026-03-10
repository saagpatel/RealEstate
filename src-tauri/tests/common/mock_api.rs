// HTTP mocking utilities for testing API integrations

use mockito::{Mock, Server, ServerGuard};
use serde_json::json;

/// Mock API server builder for Anthropic Claude API
pub struct MockApiServer {
    pub server: ServerGuard,
}

impl MockApiServer {
    /// Creates a new mock API server instance
    pub fn new() -> Self {
        MockApiServer {
            server: Server::new(),
        }
    }

    /// Returns the base URL of the mock server
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Mock response for Claude API listing analysis (non-streaming)
    /// Returns JSON with property analysis including key features
    pub fn mock_listing_analysis_response(&mut self) -> Mock {
        self.server
            .mock("POST", "/v1/messages")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&json!({
                    "id": "msg_test_analysis",
                    "type": "message",
                    "role": "assistant",
                    "content": [{
                        "type": "text",
                        "text": r#"{"key_features": ["Updated kitchen", "New roof", "Hardwood floors"], "highlighting": "Modern finishes and recent upgrades"}"#
                    }],
                    "model": "claude-sonnet-4-5-20250929",
                    "stop_reason": "end_turn",
                    "stop_sequence": null,
                    "usage": {
                        "input_tokens": 100,
                        "output_tokens": 50
                    }
                }))
                .unwrap(),
            )
            .create()
    }

    /// Mock response for streaming listing generation
    /// Returns Server-Sent Events (SSE) stream with text deltas
    pub fn mock_streaming_response(&mut self) -> Mock {
        let sse_data = r#"event: message_start
data: {"type":"message_start","message":{"id":"msg_test","type":"message","role":"assistant"}}

event: content_block_start
data: {"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"This is a beautiful property"}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" in a great neighborhood."}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" Features include modern kitchen and spacious living areas."}}

event: content_block_stop
data: {"type":"content_block_stop","index":0}

event: message_delta
data: {"type":"message_delta","delta":{"stop_reason":"end_turn"},"usage":{"output_tokens":450}}

event: message_stop
data: {"type":"message_stop"}
"#;

        self.server
            .mock("POST", "/v1/messages")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(sse_data)
            .create()
    }

    /// Mock response for LemonSqueezy license validation
    /// `is_valid`: true returns "active" status, false returns "expired"
    pub fn mock_license_validation_response(&mut self, is_valid: bool) -> Mock {
        let status = if is_valid { "active" } else { "expired" };

        self.server
            .mock(
                "GET",
                mockito::Matcher::Regex(r"^/v1/licenses.*".to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&json!({
                    "data": {
                        "id": "1",
                        "type": "licenses",
                        "attributes": {
                            "status": status,
                            "key": "test-license-key",
                            "activation_limit": 5,
                            "activation_usage": 1
                        }
                    }
                }))
                .unwrap(),
            )
            .create()
    }

    /// Mock API error response (e.g., 401 Unauthorized)
    pub fn mock_error_response(&mut self, status: usize, error_message: &str) -> Mock {
        self.server
            .mock("POST", "/v1/messages")
            .with_status(status)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&json!({
                    "error": {
                        "type": "authentication_error",
                        "message": error_message
                    }
                }))
                .unwrap(),
            )
            .create()
    }

    /// Mock rate limit error (429 status)
    pub fn mock_rate_limit_response(&mut self) -> Mock {
        self.server
            .mock("POST", "/v1/messages")
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_header("retry-after", "60")
            .with_body(
                serde_json::to_string(&json!({
                    "error": {
                        "type": "rate_limit_error",
                        "message": "Rate limit exceeded"
                    }
                }))
                .unwrap(),
            )
            .create()
    }
}

impl Default for MockApiServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_api_creation() {
        let api = MockApiServer::new();
        assert!(!api.url().is_empty(), "Mock server should have a URL");
    }

    #[test]
    fn test_mock_url_format() {
        let api = MockApiServer::new();
        let url = api.url();
        assert!(
            url.starts_with("http://"),
            "Mock URL should start with http://"
        );
    }
}

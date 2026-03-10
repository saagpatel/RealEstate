use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use tokio::time::{sleep, Duration};

use crate::error::AppError;

/// Maximum number of retries for API requests
const MAX_RETRIES: u32 = 3;

/// Initial backoff delay in milliseconds
const INITIAL_BACKOFF_MS: u64 = 1000;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum StreamEvent {
    Started {
        estimated_tokens: u32,
    },
    Delta {
        text: String,
    },
    Finished {
        full_text: String,
        input_tokens: u32,
        output_tokens: u32,
        cost_cents: u32,
    },
    Error {
        message: String,
    },
}

pub struct ClaudeClient {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct MessageRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    stream: bool,
    system: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

// SSE event parsing types
#[derive(Deserialize, Debug)]
struct SseMessageStart {
    message: SseMessageMeta,
}

#[derive(Deserialize, Debug)]
struct SseMessageMeta {
    usage: Option<SseUsage>,
}

#[derive(Deserialize, Debug)]
struct SseUsage {
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct SseContentBlockDelta {
    delta: SseDelta,
}

#[derive(Deserialize, Debug)]
struct SseDelta {
    #[serde(rename = "type", default)]
    delta_type: String,
    #[serde(default)]
    text: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    stop_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SseMessageDelta {
    usage: Option<SseUsage>,
    #[allow(dead_code)]
    delta: Option<SseDelta>,
}

#[derive(Deserialize, Debug)]
struct SseError {
    error: SseErrorDetail,
}

#[derive(Deserialize, Debug)]
struct SseErrorDetail {
    message: String,
}

impl ClaudeClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }

    /// Send a non-streaming request and return the full text response (with retry logic)
    pub async fn send_message(
        &self,
        system: &str,
        user_content: &str,
        max_tokens: u32,
    ) -> Result<(String, u32, u32), AppError> {
        let mut attempt = 0;

        loop {
            attempt += 1;

            let result = self
                .send_message_attempt(system, user_content, max_tokens)
                .await;

            match result {
                Ok(response) => return Ok(response),
                Err(e) if attempt >= MAX_RETRIES => return Err(e),
                Err(e) if is_retryable_error(&e) => {
                    let backoff_ms = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 1);
                    eprintln!(
                        "API request failed (attempt {}/{}): {}. Retrying in {}ms...",
                        attempt, MAX_RETRIES, e, backoff_ms
                    );
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
                Err(e) => return Err(e), // Non-retryable error
            }
        }
    }

    /// Internal method: single attempt of non-streaming request
    async fn send_message_attempt(
        &self,
        system: &str,
        user_content: &str,
        max_tokens: u32,
    ) -> Result<(String, u32, u32), AppError> {
        let body = MessageRequest {
            model: &self.model,
            max_tokens,
            stream: false,
            system,
            messages: vec![Message {
                role: "user",
                content: user_content,
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("content-type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(AppError::from_reqwest_error)?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            // Check for rate limit
            if status.as_u16() == 429 {
                return Err(AppError::ApiRateLimit(text));
            }

            // Check for overloaded
            if status.as_u16() == 529 {
                return Err(AppError::ApiOverloaded(text));
            }

            return Err(AppError::Api(format!("API returned {}: {}", status, text)));
        }

        let resp: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Api(format!("Failed to parse response: {}", e)))?;

        let text = resp["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let input_tokens = resp["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32;
        let output_tokens = resp["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;

        Ok((text, input_tokens, output_tokens))
    }

    /// Send a streaming request, forwarding text deltas through the Channel
    pub async fn stream_message(
        &self,
        system: &str,
        user_content: &str,
        max_tokens: u32,
        channel: &Channel<StreamEvent>,
    ) -> Result<(String, u32, u32), AppError> {
        let body = MessageRequest {
            model: &self.model,
            max_tokens,
            stream: true,
            system,
            messages: vec![Message {
                role: "user",
                content: user_content,
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("content-type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("API returned {}: {}", status, text)));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        let mut full_text = String::new();
        let mut input_tokens: u32 = 0;
        let mut output_tokens: u32 = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| AppError::Api(format!("Stream error: {}", e)))?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.push_str(&chunk_str);

            // Process complete SSE events (separated by double newlines)
            while let Some(pos) = buffer.find("\n\n") {
                let event_block = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                let (event_type, data) = parse_sse_event(&event_block);

                match event_type.as_str() {
                    "message_start" => {
                        if let Some(data) = &data {
                            if let Ok(msg) = serde_json::from_str::<SseMessageStart>(data) {
                                if let Some(usage) = msg.message.usage {
                                    input_tokens = usage.input_tokens.unwrap_or(0);
                                }
                            }
                        }
                        let _ = channel.send(StreamEvent::Started {
                            estimated_tokens: input_tokens,
                        });
                    }
                    "content_block_delta" => {
                        if let Some(data) = &data {
                            if let Ok(delta) = serde_json::from_str::<SseContentBlockDelta>(data) {
                                if delta.delta.delta_type == "text_delta" {
                                    if let Some(text) = delta.delta.text {
                                        full_text.push_str(&text);
                                        let _ = channel.send(StreamEvent::Delta { text });
                                    }
                                }
                            }
                        }
                    }
                    "message_delta" => {
                        if let Some(data) = &data {
                            if let Ok(msg) = serde_json::from_str::<SseMessageDelta>(data) {
                                if let Some(usage) = msg.usage {
                                    output_tokens = usage.output_tokens.unwrap_or(0);
                                }
                            }
                        }
                    }
                    "error" => {
                        if let Some(data) = &data {
                            if let Ok(err) = serde_json::from_str::<SseError>(data) {
                                let _ = channel.send(StreamEvent::Error {
                                    message: err.error.message.clone(),
                                });
                                return Err(AppError::Api(err.error.message));
                            }
                        }
                    }
                    // ping, content_block_start, content_block_stop, message_stop: ignore
                    _ => {}
                }
            }
        }

        Ok((full_text, input_tokens, output_tokens))
    }
}

/// Determine if an error is retryable (network, rate limit, overloaded)
fn is_retryable_error(error: &AppError) -> bool {
    matches!(
        error,
        AppError::ApiRateLimit(_) | AppError::ApiOverloaded(_) | AppError::NetworkError(_)
    )
}

/// Calculate cost in cents: (input_tokens * 3 + output_tokens * 15) / 10000
pub fn calculate_cost_cents(input_tokens: u32, output_tokens: u32) -> u32 {
    ((input_tokens as u64 * 3 + output_tokens as u64 * 15) / 10000) as u32
}

/// Parse an SSE event block into (event_type, data)
fn parse_sse_event(block: &str) -> (String, Option<String>) {
    let mut event_type = String::new();
    let mut data_lines = Vec::new();

    for line in block.lines() {
        if let Some(val) = line.strip_prefix("event: ") {
            event_type = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("data: ") {
            data_lines.push(val.to_string());
        } else if let Some(stripped) = line.strip_prefix("data:") {
            // "data:" with no space after
            data_lines.push(stripped.to_string());
        }
    }

    let data = if data_lines.is_empty() {
        None
    } else {
        Some(data_lines.join("\n"))
    };

    (event_type, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_event_message_start() {
        let block = r#"event: message_start
data: {"type":"message_start","message":{"id":"msg_123","type":"message","role":"assistant","content":[],"model":"claude-sonnet-4-5-20250929","stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":25,"output_tokens":0}}}"#;

        let (event_type, data) = parse_sse_event(block);
        assert_eq!(event_type, "message_start");
        assert!(data.is_some());

        let msg: SseMessageStart = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(msg.message.usage.unwrap().input_tokens, Some(25));
    }

    #[test]
    fn test_parse_sse_event_content_delta() {
        let block = r#"event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#;

        let (event_type, data) = parse_sse_event(block);
        assert_eq!(event_type, "content_block_delta");

        let delta: SseContentBlockDelta = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(delta.delta.delta_type, "text_delta");
        assert_eq!(delta.delta.text.unwrap(), "Hello");
    }

    #[test]
    fn test_parse_sse_event_message_delta() {
        let block = r#"event: message_delta
data: {"type":"message_delta","usage":{"output_tokens":15},"delta":{"stop_reason":"end_turn","stop_sequence":null}}"#;

        let (event_type, data) = parse_sse_event(block);
        assert_eq!(event_type, "message_delta");

        let msg: SseMessageDelta = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(msg.usage.unwrap().output_tokens, Some(15));
    }

    #[test]
    fn test_parse_sse_event_error() {
        let block = r#"event: error
data: {"type":"error","error":{"type":"overloaded_error","message":"Overloaded"}}"#;

        let (event_type, data) = parse_sse_event(block);
        assert_eq!(event_type, "error");

        let err: SseError = serde_json::from_str(&data.unwrap()).unwrap();
        assert_eq!(err.error.message, "Overloaded");
    }

    #[test]
    fn test_parse_sse_event_ping() {
        let block = "event: ping\ndata: {}";
        let (event_type, _) = parse_sse_event(block);
        assert_eq!(event_type, "ping");
    }

    #[test]
    fn test_calculate_cost_cents() {
        // 500 input * 3 = 1500, 500 output * 15 = 7500 -> 9000 / 10000 = 0
        // Typical small generation rounds to 0 cents (sub-cent)
        let cost = calculate_cost_cents(500, 500);
        assert!(cost < 2); // Sub-cent for small generations
    }
}

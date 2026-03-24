use serde::{Deserialize, Serialize};

use crate::error::AppError;

const VALIDATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/validate";
const DEVELOPMENT_BYPASS_MESSAGE: &str =
    "Development mode: license checks are bypassed locally. Release builds still require activation.";

#[derive(Debug, Serialize)]
struct ValidateRequest {
    license_key: String,
    instance_name: String,
}

#[derive(Debug, Deserialize)]
struct ValidateResponse {
    valid: bool,
    error: Option<String>,
    license_key: Option<LicenseKeyInfo>,
}

#[derive(Debug, Deserialize)]
struct LicenseKeyInfo {
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseStatus {
    pub is_valid: bool,
    pub license_key: String,
    pub validated_at: String,
    pub error: Option<String>,
}

pub fn development_license_status() -> LicenseStatus {
    LicenseStatus {
        is_valid: true,
        license_key: String::new(),
        validated_at: chrono::Utc::now().to_rfc3339(),
        error: Some(DEVELOPMENT_BYPASS_MESSAGE.to_string()),
    }
}

/// Validate a license key against LemonSqueezy API
pub async fn validate_license(license_key: &str) -> Result<LicenseStatus, AppError> {
    if license_key.is_empty() {
        return Ok(LicenseStatus {
            is_valid: false,
            license_key: String::new(),
            validated_at: chrono::Utc::now().to_rfc3339(),
            error: Some("No license key provided".to_string()),
        });
    }

    let client = reqwest::Client::new();

    let instance_name = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let response = client
        .post(VALIDATE_URL)
        .json(&ValidateRequest {
            license_key: license_key.to_string(),
            instance_name,
        })
        .send()
        .await;

    match response {
        Ok(resp) => {
            if let Ok(body) = resp.json::<ValidateResponse>().await {
                let status = body
                    .license_key
                    .as_ref()
                    .map(|lk| lk.status.as_str())
                    .unwrap_or("unknown");

                Ok(LicenseStatus {
                    is_valid: body.valid && status == "active",
                    license_key: license_key.to_string(),
                    validated_at: chrono::Utc::now().to_rfc3339(),
                    error: if body.valid {
                        None
                    } else {
                        body.error
                            .or_else(|| Some(format!("License status: {}", status)))
                    },
                })
            } else {
                Ok(LicenseStatus {
                    is_valid: false,
                    license_key: license_key.to_string(),
                    validated_at: chrono::Utc::now().to_rfc3339(),
                    error: Some("Failed to parse validation response".to_string()),
                })
            }
        }
        Err(_) => {
            // Network error — allow grace period (treat as valid for offline use)
            Ok(LicenseStatus {
                is_valid: true,
                license_key: license_key.to_string(),
                validated_at: chrono::Utc::now().to_rfc3339(),
                error: Some("Could not reach license server. Working in offline mode.".to_string()),
            })
        }
    }
}

/// Check if a cached validation is still fresh (within 24 hours)
pub fn is_cache_valid(validated_at: &str) -> bool {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(validated_at) {
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(dt);
        elapsed.num_hours() < 24
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cache_valid_recent() {
        let recent = chrono::Utc::now().to_rfc3339();
        assert!(is_cache_valid(&recent));
    }

    #[test]
    fn test_is_cache_valid_expired() {
        let old = (chrono::Utc::now() - chrono::Duration::hours(25)).to_rfc3339();
        assert!(!is_cache_valid(&old));
    }

    #[test]
    fn test_is_cache_valid_invalid_string() {
        assert!(!is_cache_valid("not-a-date"));
    }

    #[test]
    fn test_development_license_status_allows_local_debug_access() {
        let status = development_license_status();

        assert!(status.is_valid);
        assert!(status.license_key.is_empty());
        assert_eq!(
            status.error.as_deref(),
            Some(DEVELOPMENT_BYPASS_MESSAGE)
        );
        assert!(!status.validated_at.is_empty());
    }
}

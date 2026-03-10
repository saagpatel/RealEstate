use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("API key is missing or invalid. Please set your Anthropic API key in Settings (must start with 'sk-ant-').")]
    MissingApiKey,

    #[error(
        "API rate limit exceeded. Please wait a few minutes before generating again. Error: {0}"
    )]
    ApiRateLimit(String),

    #[error(
        "Anthropic API is currently overloaded. Please wait a moment and try again. Error: {0}"
    )]
    ApiOverloaded(String),

    #[error("Failed to connect to Anthropic API. Please check your internet connection and try again. Error: {0}")]
    ApiConnectionError(String),

    #[error(
        "Network error occurred. Please check your internet connection and try again. Error: {0}"
    )]
    NetworkError(String),

    #[error("API request failed: {0}")]
    Api(String),

    #[error("Property not found (ID: {0}). It may have been deleted.")]
    PropertyNotFound(String),

    #[error("Listing not found (ID: {0}). It may have been deleted.")]
    ListingNotFound(String),

    #[error("Brand voice not found (ID: {0}). It may have been deleted.")]
    BrandVoiceNotFound(String),

    #[error(
        "License key is invalid or expired. Please purchase or renew at https://lemonsqueezy.com"
    )]
    InvalidLicense,

    #[error(
        "Failed to validate license key. Please check your internet connection or try again later."
    )]
    LicenseValidationFailed,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to export to PDF: {0}. Please ensure you have write permissions.")]
    ExportPdfFailed(String),

    #[error("Failed to export to DOCX: {0}. Please ensure you have write permissions.")]
    ExportDocxFailed(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Failed to import photos: {0}. Please ensure files are valid images (JPEG, PNG).")]
    PhotoImportFailed(String),

    #[error("Maximum of 20 photos per property. Please delete some photos before adding more.")]
    PhotoLimitExceeded,

    #[error("Photo error: {0}")]
    Photo(String),

    #[error("Configuration error: {0}. Please check your settings.")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid property data: {0}")]
    InvalidPropertyData(String),

    #[error("Failed to parse listing content: {0}. Please try generating again.")]
    ListingParseFailed(String),

    #[error("Generation failed: {0}. Try with different style/tone settings.")]
    GenerationFailed(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Network timeout. Please check your connection and try again.")]
    NetworkTimeout,

    #[error("Request failed: {0}")]
    RequestFailed(String),
}

#[allow(dead_code)]
impl AppError {
    /// Convert from reqwest::Error with context
    pub fn from_reqwest_error(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::NetworkTimeout
        } else if err.is_connect() {
            AppError::NetworkError(err.to_string())
        } else if err.is_status() {
            if let Some(status) = err.status() {
                match status.as_u16() {
                    429 => AppError::ApiRateLimit(err.to_string()),
                    529 => AppError::ApiOverloaded(err.to_string()),
                    _ => AppError::RequestFailed(format!("HTTP {}: {}", status, err)),
                }
            } else {
                AppError::RequestFailed(err.to_string())
            }
        } else {
            AppError::NetworkError(err.to_string())
        }
    }

    /// Check if this error is a network issue (retryable)
    pub fn is_network_error(&self) -> bool {
        matches!(
            self,
            AppError::NetworkTimeout | AppError::NetworkError(_) | AppError::ApiConnectionError(_)
        )
    }

    /// Check if this error is due to rate limiting
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, AppError::ApiRateLimit(_))
    }

    /// Check if this error is due to API overload
    pub fn is_overloaded(&self) -> bool {
        matches!(self, AppError::ApiOverloaded(_))
    }
}

// Tauri requires Serialize for command return errors
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

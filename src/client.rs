use crate::error::{CufinderError, Result};
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use std::time::Duration;

/// Configuration for the CUFinder client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.cufinder.io/v2".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

/// HTTP client for CUFinder API
#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
    http_client: ReqwestClient,
}

impl Client {
    /// Create a new client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http_client = ReqwestClient::builder()
            .timeout(config.timeout)
            .build()
            .map_err(CufinderError::HttpError)?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create a new client with just an API key
    pub fn with_api_key(api_key: String) -> Result<Self> {
        Self::new(ClientConfig {
            api_key,
            ..Default::default()
        })
    }

    /// Send a POST request to the API
    pub async fn post<T>(&self, endpoint: &str, data: &T) -> Result<serde_json::Value>
    where
        T: Serialize,
    {
        let url = format!("{}{}", self.config.base_url, endpoint);
        
        // Convert data to form-encoded format
        let form_data = serde_urlencoded::to_string(data)
            .map_err(|e| CufinderError::ValidationError(format!("Failed to encode form data: {}", e)))?;
        
        let response = self
            .http_client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "cufinder-rust/0.0.1")
            .body(form_data)
            .send()
            .await
            .map_err(CufinderError::HttpError)?;

        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            return match status.as_u16() {
                401 => Err(CufinderError::AuthenticationError(error_text)),
                429 => Err(CufinderError::RateLimitError(error_text)),
                402 => Err(CufinderError::CreditLimitError(error_text)),
                _ => Err(CufinderError::ApiError {
                    status: status.as_u16(),
                    message: error_text,
                }),
            };
        }

        let mut json_response: serde_json::Value = response
            .json()
            .await
            .map_err(CufinderError::HttpError)?;

        // Check if the response has a "data" wrapper and extract it
        if let Some(data_wrapper) = json_response.get("data") {
            if let Some(meta_data) = json_response.get("meta_data") {
                // Create a new object with data content plus meta_data
                let mut data_obj = data_wrapper.clone();
                if let serde_json::Value::Object(ref mut map) = data_obj {
                    map.insert("meta_data".to_string(), meta_data.clone());
                }
                json_response = data_obj;
            } else {
                json_response = data_wrapper.clone();
            }
        }

        Ok(json_response)
    }

    /// Get the underlying HTTP client for advanced usage
    pub fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }

    /// Get the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
}

use crate::error::{CufinderError, Result};
use reqwest::{Client as ReqwestClient, header};
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
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| CufinderError::ValidationError(format!("Invalid API key: {}", e)))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let http_client = ReqwestClient::builder()
            .timeout(config.timeout)
            .default_headers(headers)
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
        
        let response = self
            .http_client
            .post(&url)
            .json(data)
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

        let json_response: serde_json::Value = response
            .json()
            .await
            .map_err(CufinderError::HttpError)?;

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

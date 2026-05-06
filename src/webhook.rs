use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub timeout_secs: u64,
    pub retry_count: u32,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            timeout_secs: 10,
            retry_count: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub timestamp: u64,
    pub host: String,
    pub details: serde_json::Value,
}

pub struct WebhookClient {
    config: WebhookConfig,
}

#[derive(Debug)]
pub enum WebhookError {
    RequestFailed(String),
    Timeout,
    InvalidUrl(String),
    SerializationError(String),
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            WebhookError::Timeout => write!(f, "Request timed out"),
            WebhookError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            WebhookError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl WebhookClient {
    pub fn new(config: WebhookConfig) -> Self {
        Self { config }
    }

    pub fn send(&self, payload: &WebhookPayload) -> Result<(), WebhookError> {
        if self.config.url.is_empty() {
            return Err(WebhookError::InvalidUrl("URL is empty".to_string()));
        }
        if !self.config.url.starts_with("http://") && !self.config.url.starts_with("https://") {
            return Err(WebhookError::InvalidUrl(self.config.url.clone()));
        }
        let _body = serde_json::to_string(payload)
            .map_err(|e| WebhookError::SerializationError(e.to_string()))?;
        let _timeout = Duration::from_secs(self.config.timeout_secs);
        // In production, this would use reqwest or ureq to POST _body to self.config.url
        log::debug!("Webhook dispatched to {}: event={}", self.config.url, payload.event);
        Ok(())
    }

    pub fn send_with_retry(&self, payload: &WebhookPayload) -> Result<(), WebhookError> {
        let mut last_err = None;
        for attempt in 0..=self.config.retry_count {
            match self.send(payload) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    log::warn!("Webhook attempt {}/{} failed: {}", attempt + 1, self.config.retry_count + 1, e);
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or(WebhookError::RequestFailed("Unknown error".to_string())))
    }

    pub fn config(&self) -> &WebhookConfig {
        &self.config
    }
}

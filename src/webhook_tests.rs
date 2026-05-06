#[cfg(test)]
mod tests {
    use super::super::webhook::*;
    use std::collections::HashMap;

    fn sample_payload() -> WebhookPayload {
        WebhookPayload {
            event: "port_opened".to_string(),
            timestamp: 1700000000,
            host: "localhost".to_string(),
            details: serde_json::json!({"port": 8080, "protocol": "tcp"}),
        }
    }

    #[test]
    fn test_default_config() {
        let cfg = WebhookConfig::default();
        assert_eq!(cfg.method, "POST");
        assert_eq!(cfg.timeout_secs, 10);
        assert_eq!(cfg.retry_count, 3);
        assert!(cfg.url.is_empty());
    }

    #[test]
    fn test_send_empty_url_fails() {
        let client = WebhookClient::new(WebhookConfig::default());
        let payload = sample_payload();
        let result = client.send(&payload);
        assert!(matches!(result, Err(WebhookError::InvalidUrl(_))));
    }

    #[test]
    fn test_send_invalid_url_scheme_fails() {
        let cfg = WebhookConfig {
            url: "ftp://example.com/hook".to_string(),
            ..Default::default()
        };
        let client = WebhookClient::new(cfg);
        let result = client.send(&sample_payload());
        assert!(matches!(result, Err(WebhookError::InvalidUrl(_))));
    }

    #[test]
    fn test_send_valid_https_url_succeeds() {
        let cfg = WebhookConfig {
            url: "https://hooks.example.com/portwatch".to_string(),
            ..Default::default()
        };
        let client = WebhookClient::new(cfg);
        let result = client.send(&sample_payload());
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_valid_http_url_succeeds() {
        let cfg = WebhookConfig {
            url: "http://localhost:9000/notify".to_string(),
            ..Default::default()
        };
        let client = WebhookClient::new(cfg);
        assert!(client.send(&sample_payload()).is_ok());
    }

    #[test]
    fn test_send_with_retry_succeeds_on_valid_url() {
        let cfg = WebhookConfig {
            url: "https://hooks.example.com/portwatch".to_string(),
            retry_count: 2,
            ..Default::default()
        };
        let client = WebhookClient::new(cfg);
        assert!(client.send_with_retry(&sample_payload()).is_ok());
    }

    #[test]
    fn test_config_accessor() {
        let cfg = WebhookConfig {
            url: "https://example.com".to_string(),
            timeout_secs: 5,
            ..Default::default()
        };
        let client = WebhookClient::new(cfg.clone());
        assert_eq!(client.config().url, "https://example.com");
        assert_eq!(client.config().timeout_secs, 5);
    }

    #[test]
    fn test_custom_headers_preserved() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        let cfg = WebhookConfig {
            url: "https://example.com/hook".to_string(),
            headers: headers.clone(),
            ..Default::default()
        };
        let client = WebhookClient::new(cfg);
        assert_eq!(client.config().headers.get("Authorization").unwrap(), "Bearer token123");
    }
}

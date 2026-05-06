use crate::webhook::{WebhookClient, WebhookConfig, WebhookError, WebhookPayload};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WebhookDispatcher {
    clients: Vec<WebhookClient>,
    dispatch_log: Arc<Mutex<Vec<DispatchRecord>>>,
}

#[derive(Debug, Clone)]
pub struct DispatchRecord {
    pub event: String,
    pub url: String,
    pub success: bool,
    pub timestamp: u64,
}

impl WebhookDispatcher {
    pub fn new(configs: Vec<WebhookConfig>) -> Self {
        let clients = configs.into_iter().map(WebhookClient::new).collect();
        Self {
            clients,
            dispatch_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn dispatch(&self, event: &str, details: serde_json::Value) -> Vec<Result<(), WebhookError>> {
        let payload = WebhookPayload {
            event: event.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            host: hostname(),
            details,
        };

        let mut results = Vec::new();
        for client in &self.clients {
            let result = client.send_with_retry(&payload);
            let success = result.is_ok();
            if let Ok(mut log) = self.dispatch_log.lock() {
                log.push(DispatchRecord {
                    event: event.to_string(),
                    url: client.config().url.clone(),
                    success,
                    timestamp: payload.timestamp,
                });
            }
            results.push(result);
        }
        results
    }

    pub fn dispatch_port_change(&self, port: u16, protocol: &str, change_type: &str) -> Vec<Result<(), WebhookError>> {
        let details = serde_json::json!({
            "port": port,
            "protocol": protocol,
            "change": change_type,
        });
        self.dispatch(change_type, details)
    }

    pub fn records(&self) -> Vec<DispatchRecord> {
        self.dispatch_log.lock().map(|l| l.clone()).unwrap_or_default()
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}

fn hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}

//! Suppression rules: temporarily silence alerts for known/expected port changes.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct SuppressionRule {
    pub port: u16,
    pub protocol: String,
    pub reason: String,
    pub expires_at: SystemTime,
}

impl SuppressionRule {
    pub fn new(port: u16, protocol: &str, reason: &str, duration: Duration) -> Self {
        Self {
            port,
            protocol: protocol.to_string(),
            reason: reason.to_string(),
            expires_at: SystemTime::now() + duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }
}

#[derive(Debug, Default)]
pub struct SuppressionStore {
    rules: HashMap<(u16, String), SuppressionRule>,
}

impl SuppressionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, rule: SuppressionRule) {
        let key = (rule.port, rule.protocol.clone());
        self.rules.insert(key, rule);
    }

    pub fn is_suppressed(&self, port: u16, protocol: &str) -> bool {
        let key = (port, protocol.to_string());
        match self.rules.get(&key) {
            Some(rule) => !rule.is_expired(),
            None => false,
        }
    }

    pub fn remove(&mut self, port: u16, protocol: &str) {
        let key = (port, protocol.to_string());
        self.rules.remove(&key);
    }

    /// Purge all expired rules, returning the count removed.
    pub fn purge_expired(&mut self) -> usize {
        let before = self.rules.len();
        self.rules.retain(|_, rule| !rule.is_expired());
        before - self.rules.len()
    }

    pub fn active_rules(&self) -> Vec<&SuppressionRule> {
        self.rules.values().filter(|r| !r.is_expired()).collect()
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

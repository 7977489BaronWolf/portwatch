//! Escalation module: tracks repeated alerts and escalates notifications
//! when the same port change fires beyond a configured threshold.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct EscalationConfig {
    /// Number of repeated alerts before escalation triggers
    pub threshold: u32,
    /// Window within which repeated alerts are counted
    pub window: Duration,
}

impl Default for EscalationConfig {
    fn default() -> Self {
        Self {
            threshold: 3,
            window: Duration::from_secs(300),
        }
    }
}

#[derive(Debug)]
struct AlertRecord {
    count: u32,
    first_seen: Instant,
    escalated: bool,
}

#[derive(Debug, Default)]
pub struct EscalationTracker {
    config: EscalationConfig,
    records: HashMap<String, AlertRecord>,
}

impl EscalationTracker {
    pub fn new(config: EscalationConfig) -> Self {
        Self {
            config,
            records: HashMap::new(),
        }
    }

    /// Record an alert for `key` (e.g. "port:8080:opened").
    /// Returns `true` if this alert should be escalated.
    pub fn record(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let config = &self.config;

        let record = self.records.entry(key.to_string()).or_insert(AlertRecord {
            count: 0,
            first_seen: now,
            escalated: false,
        });

        // Reset window if expired
        if now.duration_since(record.first_seen) > config.window {
            record.count = 0;
            record.first_seen = now;
            record.escalated = false;
        }

        record.count += 1;

        if record.count >= config.threshold && !record.escalated {
            record.escalated = true;
            return true;
        }

        false
    }

    /// Clear escalation state for a key (e.g. after manual acknowledgement).
    pub fn acknowledge(&mut self, key: &str) {
        self.records.remove(key);
    }

    /// Return current alert count for a key within the active window.
    pub fn count(&self, key: &str) -> u32 {
        self.records.get(key).map(|r| r.count).unwrap_or(0)
    }
}

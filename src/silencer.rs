//! Silencer module: temporarily silence alerts for specific ports or port ranges.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SilenceRule {
    pub port: u16,
    pub reason: String,
    pub expires_at: Instant,
}

#[derive(Debug, Default)]
pub struct Silencer {
    rules: HashMap<u16, SilenceRule>,
}

impl Silencer {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Silence alerts for a given port for the specified duration.
    pub fn silence(&mut self, port: u16, duration: Duration, reason: &str) {
        let rule = SilenceRule {
            port,
            reason: reason.to_string(),
            expires_at: Instant::now() + duration,
        };
        self.rules.insert(port, rule);
    }

    /// Remove a silence rule for a port immediately.
    pub fn unsilence(&mut self, port: u16) -> bool {
        self.rules.remove(&port).is_some()
    }

    /// Check whether alerts for a given port are currently silenced.
    pub fn is_silenced(&mut self, port: u16) -> bool {
        if let Some(rule) = self.rules.get(&port) {
            if Instant::now() < rule.expires_at {
                return true;
            }
            // Expired — remove lazily
            self.rules.remove(&port);
        }
        false
    }

    /// Purge all expired silence rules.
    pub fn purge_expired(&mut self) {
        let now = Instant::now();
        self.rules.retain(|_, rule| now < rule.expires_at);
    }

    /// Return a snapshot of currently active silence rules.
    pub fn active_rules(&mut self) -> Vec<SilenceRule> {
        self.purge_expired();
        self.rules.values().cloned().collect()
    }

    /// Return the number of active silence rules.
    pub fn active_count(&mut self) -> usize {
        self.purge_expired();
        self.rules.len()
    }
}

use crate::config::Config;
use crate::diff_engine::PortDiff;
use crate::notifier::Notifier;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Manages alert throttling and dispatching notifications for port changes.
pub struct AlertManager {
    notifier: Notifier,
    cooldown: Duration,
    last_alert: HashMap<String, Instant>,
}

impl AlertManager {
    pub fn new(config: &Config) -> Self {
        AlertManager {
            notifier: Notifier::new(config),
            cooldown: Duration::from_secs(config.alert_cooldown_secs.unwrap_or(60)),
            last_alert: HashMap::new(),
        }
    }

    /// Process a list of diffs and fire alerts for changes that are not throttled.
    pub fn process_diffs(&mut self, diffs: &[PortDiff]) {
        for diff in diffs {
            let key = diff.key();
            let should_alert = self
                .last_alert
                .get(&key)
                .map(|t| t.elapsed() >= self.cooldown)
                .unwrap_or(true);

            if should_alert {
                let message = self.format_message(diff);
                if let Err(e) = self.notifier.send(&message) {
                    eprintln!("[alert_manager] Failed to send notification: {}", e);
                } else {
                    self.last_alert.insert(key, Instant::now());
                }
            }
        }
    }

    fn format_message(&self, diff: &PortDiff) -> String {
        match diff {
            PortDiff::Opened(entry) => {
                format!("[portwatch] Port OPENED: {} ({}) PID={:?}", entry.port, entry.protocol, entry.pid)
            }
            PortDiff::Closed(entry) => {
                format!("[portwatch] Port CLOSED: {} ({}) PID={:?}", entry.port, entry.protocol, entry.pid)
            }
        }
    }

    /// Clear throttle state (useful for testing or forced re-alert).
    pub fn reset_throttle(&mut self) {
        self.last_alert.clear();
    }
}

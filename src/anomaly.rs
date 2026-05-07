//! Anomaly detection for port scan results.
//!
//! Detects statistically unusual port activity based on historical
//! frequency data, flagging ports that open/close more often than expected.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AnomalyKind {
    FrequentOpen,
    FrequentClose,
    UnexpectedProtocol,
}

#[derive(Debug, Clone)]
pub struct Anomaly {
    pub port: u16,
    pub kind: AnomalyKind,
    pub score: f64,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct AnomalyDetector {
    /// port -> number of times it has changed state
    change_counts: HashMap<u16, u32>,
    /// threshold: changes per observation window before flagging
    threshold: u32,
    /// known protocol map: port -> expected protocol label
    expected_protocols: HashMap<u16, String>,
}

impl AnomalyDetector {
    pub fn new(threshold: u32, expected_protocols: HashMap<u16, String>) -> Self {
        Self {
            change_counts: HashMap::new(),
            threshold,
            expected_protocols,
        }
    }

    /// Record a state change for the given port and return any anomaly detected.
    pub fn record_change(&mut self, port: u16, protocol: &str) -> Option<Anomaly> {
        let count = self.change_counts.entry(port).or_insert(0);
        *count += 1;

        // Check protocol mismatch first
        if let Some(expected) = self.expected_protocols.get(&port) {
            if expected != protocol {
                return Some(Anomaly {
                    port,
                    kind: AnomalyKind::UnexpectedProtocol,
                    score: 1.0,
                    description: format!(
                        "Port {} expected protocol '{}', got '{}'",
                        port, expected, protocol
                    ),
                });
            }
        }

        // Check frequency threshold
        if *count >= self.threshold {
            let score = (*count as f64) / (self.threshold as f64);
            return Some(Anomaly {
                port,
                kind: AnomalyKind::FrequentOpen,
                score,
                description: format!(
                    "Port {} has changed state {} times (threshold: {})",
                    port, count, self.threshold
                ),
            });
        }

        None
    }

    pub fn reset_counts(&mut self) {
        self.change_counts.clear();
    }

    pub fn change_count(&self, port: u16) -> u32 {
        *self.change_counts.get(&port).unwrap_or(&0)
    }
}

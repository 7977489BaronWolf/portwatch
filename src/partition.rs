//! Partition module: splits alert streams into named buckets based on criteria.

use std::collections::HashMap;
use crate::port_scanner::PortEntry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionKey {
    Protocol(String),
    PortRange(u16, u16),
    Custom(String),
}

impl PartitionKey {
    pub fn label(&self) -> String {
        match self {
            PartitionKey::Protocol(p) => format!("proto:{}", p),
            PartitionKey::PortRange(lo, hi) => format!("range:{}-{}", lo, hi),
            PartitionKey::Custom(s) => format!("custom:{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartitionRule {
    pub key: PartitionKey,
    pub description: String,
}

impl PartitionRule {
    pub fn new(key: PartitionKey, description: impl Into<String>) -> Self {
        Self { key, description: description.into() }
    }

    pub fn matches(&self, entry: &PortEntry) -> bool {
        match &self.key {
            PartitionKey::Protocol(p) => entry.protocol.eq_ignore_ascii_case(p),
            PartitionKey::PortRange(lo, hi) => entry.port >= *lo && entry.port <= *hi,
            PartitionKey::Custom(_) => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Partitioner {
    rules: Vec<PartitionRule>,
}

impl Partitioner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: PartitionRule) {
        self.rules.push(rule);
    }

    /// Partition entries into buckets keyed by partition label.
    /// Entries matching no rule go into "unclassified".
    pub fn partition(&self, entries: &[PortEntry]) -> HashMap<String, Vec<PortEntry>> {
        let mut buckets: HashMap<String, Vec<PortEntry>> = HashMap::new();
        for entry in entries {
            let mut matched = false;
            for rule in &self.rules {
                if rule.matches(entry) {
                    buckets
                        .entry(rule.key.label())
                        .or_default()
                        .push(entry.clone());
                    matched = true;
                    break;
                }
            }
            if !matched {
                buckets
                    .entry("unclassified".to_string())
                    .or_default()
                    .push(entry.clone());
            }
        }
        buckets
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

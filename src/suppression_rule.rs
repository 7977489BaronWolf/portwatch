use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuppressionCondition {
    PortRange(u16, u16),
    ExactPort(u16),
    Protocol(String),
    Tag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionRule {
    pub id: String,
    pub name: String,
    pub conditions: Vec<SuppressionCondition>,
    pub expires_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

impl SuppressionRule {
    pub fn new(id: impl Into<String>, name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            conditions: Vec::new(),
            expires_at: None,
            reason: reason.into(),
            created_at: Utc::now(),
        }
    }

    pub fn with_condition(mut self, condition: SuppressionCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_ttl(mut self, duration: Duration) -> Self {
        self.expires_at = Some(Utc::now() + duration);
        self
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|exp| Utc::now() > exp).unwrap_or(false)
    }

    pub fn matches_port(&self, port: u16, protocol: &str, tags: &[String]) -> bool {
        if self.is_expired() {
            return false;
        }
        if self.conditions.is_empty() {
            return true;
        }
        self.conditions.iter().any(|cond| match cond {
            SuppressionCondition::ExactPort(p) => *p == port,
            SuppressionCondition::PortRange(lo, hi) => port >= *lo && port <= *hi,
            SuppressionCondition::Protocol(proto) => proto.eq_ignore_ascii_case(protocol),
            SuppressionCondition::Tag(tag) => tags.iter().any(|t| t == tag),
        })
    }
}

#[derive(Debug, Default)]
pub struct SuppressionRuleStore {
    rules: HashMap<String, SuppressionRule>,
}

impl SuppressionRuleStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, rule: SuppressionRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn remove(&mut self, id: &str) -> Option<SuppressionRule> {
        self.rules.remove(id)
    }

    pub fn is_suppressed(&self, port: u16, protocol: &str, tags: &[String]) -> bool {
        self.rules.values().any(|r| r.matches_port(port, protocol, tags))
    }

    pub fn purge_expired(&mut self) -> usize {
        let before = self.rules.len();
        self.rules.retain(|_, r| !r.is_expired());
        before - self.rules.len()
    }

    pub fn list_active(&self) -> Vec<&SuppressionRule> {
        self.rules.values().filter(|r| !r.is_expired()).collect()
    }
}

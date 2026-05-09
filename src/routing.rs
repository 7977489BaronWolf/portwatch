//! Routing module: maps alert events to notification channels based on rules.

use crate::config::Config;
use crate::notifier::NotificationChannel;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RoutingRule {
    pub name: String,
    pub match_severity: Option<String>,
    pub match_tag: Option<String>,
    pub channels: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Router {
    rules: Vec<RoutingRule>,
    channel_map: HashMap<String, NotificationChannel>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            rules: Vec::new(),
            channel_map: HashMap::new(),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        let mut router = Router::new();
        for rule in &config.routing_rules {
            router.add_rule(rule.clone());
        }
        router
    }

    pub fn add_rule(&mut self, rule: RoutingRule) {
        self.rules.push(rule);
    }

    pub fn register_channel(&mut self, name: String, channel: NotificationChannel) {
        self.channel_map.insert(name, channel);
    }

    /// Returns the list of channel names that match the given severity and tags.
    pub fn resolve(&self, severity: &str, tags: &[String]) -> Vec<String> {
        let mut matched = Vec::new();
        for rule in &self.rules {
            let severity_ok = rule
                .match_severity
                .as_deref()
                .map(|s| s == severity)
                .unwrap_or(true);
            let tag_ok = rule
                .match_tag
                .as_ref()
                .map(|t| tags.contains(t))
                .unwrap_or(true);
            if severity_ok && tag_ok {
                for ch in &rule.channels {
                    if !matched.contains(ch) {
                        matched.push(ch.clone());
                    }
                }
            }
        }
        matched
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

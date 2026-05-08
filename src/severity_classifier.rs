use crate::severity::Severity;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SeverityRule {
    pub port_range: Option<(u16, u16)>,
    pub protocol: Option<String>,
    pub severity: Severity,
}

#[derive(Debug, Default)]
pub struct SeverityClassifier {
    rules: Vec<SeverityRule>,
    port_overrides: HashMap<u16, Severity>,
}

impl SeverityClassifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: SeverityRule) {
        self.rules.push(rule);
    }

    pub fn set_port_override(&mut self, port: u16, severity: Severity) {
        self.port_overrides.insert(port, severity);
    }

    pub fn classify(&self, port: u16, protocol: &str) -> Severity {
        if let Some(sev) = self.port_overrides.get(&port) {
            return sev.clone();
        }
        for rule in &self.rules {
            if let Some((lo, hi)) = rule.port_range {
                if port < lo || port > hi {
                    continue;
                }
            }
            if let Some(ref proto) = rule.protocol {
                if proto.to_lowercase() != protocol.to_lowercase() {
                    continue;
                }
            }
            return rule.severity.clone();
        }
        Severity::Info
    }

    pub fn classify_new_port(port: u16) -> Severity {
        match port {
            1..=1023 => Severity::High,
            1024..=49151 => Severity::Medium,
            _ => Severity::Low,
        }
    }
}

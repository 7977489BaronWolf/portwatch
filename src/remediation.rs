//! Remediation module: suggests or executes automated responses to port anomalies.

use crate::diff_engine::PortDiff;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RemediationAction {
    KillProcess(u32),
    BlockPort(u16),
    NotifyOnly,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct RemediationRule {
    pub port: u16,
    pub action: RemediationAction,
}

#[derive(Debug, Default)]
pub struct RemediationEngine {
    rules: HashMap<u16, RemediationAction>,
    dry_run: bool,
}

impl RemediationEngine {
    pub fn new(dry_run: bool) -> Self {
        Self {
            rules: HashMap::new(),
            dry_run,
        }
    }

    pub fn add_rule(&mut self, rule: RemediationRule) {
        self.rules.insert(rule.port, rule.action);
    }

    pub fn evaluate(&self, diff: &PortDiff) -> Vec<RemediationResult> {
        let mut results = Vec::new();
        for port in &diff.opened {
            let action = self
                .rules
                .get(port)
                .cloned()
                .unwrap_or(RemediationAction::NotifyOnly);
            results.push(RemediationResult {
                port: *port,
                action: action.clone(),
                executed: !self.dry_run && action != RemediationAction::NotifyOnly,
                dry_run: self.dry_run,
            });
        }
        results
    }
}

#[derive(Debug, Clone)]
pub struct RemediationResult {
    pub port: u16,
    pub action: RemediationAction,
    pub executed: bool,
    pub dry_run: bool,
}

impl RemediationResult {
    pub fn summary(&self) -> String {
        let status = if self.dry_run {
            "[DRY-RUN]"
        } else if self.executed {
            "[EXECUTED]"
        } else {
            "[SKIPPED]"
        };
        format!("{} port={} action={:?}", status, self.port, self.action)
    }
}

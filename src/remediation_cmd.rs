//! CLI command handler for remediation management.

use crate::config::Config;
use crate::diff_engine::PortDiff;
use crate::remediation::{RemediationAction, RemediationEngine, RemediationRule};

pub struct RemediationCmd;

impl RemediationCmd {
    /// Build a RemediationEngine from the provided config.
    pub fn build_engine(config: &Config) -> RemediationEngine {
        let dry_run = config
            .get("remediation.dry_run")
            .map(|v| v == "true")
            .unwrap_or(true);

        let mut engine = RemediationEngine::new(dry_run);

        // Load rules from config entries like "remediation.rule.<port>=<action>"
        for (key, value) in config.entries() {
            if let Some(port_str) = key.strip_prefix("remediation.rule.") {
                if let Ok(port) = port_str.parse::<u16>() {
                    let action = Self::parse_action(port, value);
                    engine.add_rule(RemediationRule { port, action });
                }
            }
        }

        engine
    }

    fn parse_action(port: u16, value: &str) -> RemediationAction {
        if value.starts_with("kill:") {
            if let Ok(pid) = value[5..].parse::<u32>() {
                return RemediationAction::KillProcess(pid);
            }
        } else if value == "block" {
            return RemediationAction::BlockPort(port);
        } else if value.starts_with("custom:") {
            return RemediationAction::Custom(value[7..].to_string());
        }
        RemediationAction::NotifyOnly
    }

    /// Run remediation against a diff and print results.
    pub fn run(config: &Config, diff: &PortDiff) {
        let engine = Self::build_engine(config);
        let results = engine.evaluate(diff);
        if results.is_empty() {
            println!("remediation: no new ports to evaluate.");
            return;
        }
        for result in &results {
            println!("{}", result.summary());
        }
    }
}

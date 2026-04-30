use std::collections::HashSet;
use crate::baseline::Baseline;
use crate::port_scanner::PortInfo;

#[derive(Debug, Clone)]
pub struct BaselineViolation {
    pub unexpected: HashSet<u16>,
    pub missing: HashSet<u16>,
}

impl BaselineViolation {
    pub fn is_clean(&self) -> bool {
        self.unexpected.is_empty() && self.missing.is_empty()
    }

    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if !self.unexpected.is_empty() {
            let mut ports: Vec<u16> = self.unexpected.iter().copied().collect();
            ports.sort();
            parts.push(format!("Unexpected ports: {:?}", ports));
        }
        if !self.missing.is_empty() {
            let mut ports: Vec<u16> = self.missing.iter().copied().collect();
            ports.sort();
            parts.push(format!("Missing ports: {:?}", ports));
        }
        if parts.is_empty() {
            "No baseline violations".to_string()
        } else {
            parts.join("; ")
        }
    }
}

pub struct BaselineChecker {
    pub alert_on_missing: bool,
    pub alert_on_unexpected: bool,
}

impl Default for BaselineChecker {
    fn default() -> Self {
        Self {
            alert_on_missing: false,
            alert_on_unexpected: true,
        }
    }
}

impl BaselineChecker {
    pub fn check(&self, baseline: &Baseline, current: &[PortInfo]) -> BaselineViolation {
        let current_ports: HashSet<u16> = current.iter().map(|p| p.port).collect();
        let unexpected = if self.alert_on_unexpected {
            baseline.unexpected_ports(&current_ports)
        } else {
            HashSet::new()
        };
        let missing = if self.alert_on_missing {
            baseline.missing_ports(&current_ports)
        } else {
            HashSet::new()
        };
        BaselineViolation { unexpected, missing }
    }
}

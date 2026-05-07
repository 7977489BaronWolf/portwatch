//! Triage module: classifies port change events by severity and suggested action.

use crate::diff_engine::PortDiff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone)]
pub struct TriageResult {
    pub diff: PortDiff,
    pub severity: Severity,
    pub action: String,
}

/// Well-known high-risk ports that warrant Critical severity when opened unexpectedly.
const CRITICAL_PORTS: &[u16] = &[22, 23, 3389, 5900, 4444, 1337, 31337];

/// Ports that are typically privileged and warrant High severity.
const HIGH_PORTS: &[u16] = &[21, 25, 53, 80, 110, 143, 443, 8080, 8443];

pub fn triage(diff: &PortDiff) -> TriageResult {
    let port = diff.port;
    let is_opened = diff.is_opened;

    let severity = if is_opened {
        if CRITICAL_PORTS.contains(&port) {
            Severity::Critical
        } else if HIGH_PORTS.contains(&port) {
            Severity::High
        } else if port < 1024 {
            Severity::Medium
        } else {
            Severity::Low
        }
    } else {
        // Closed ports are generally informational unless they were critical
        if CRITICAL_PORTS.contains(&port) {
            Severity::Medium
        } else {
            Severity::Info
        }
    };

    let action = match severity {
        Severity::Critical => format!("IMMEDIATE: Investigate port {} — known high-risk service", port),
        Severity::High => format!("REVIEW: Port {} opened on a privileged/common service port", port),
        Severity::Medium => format!("MONITOR: Port {} change detected; verify intent", port),
        Severity::Low => format!("LOG: Unprivileged port {} opened; low risk", port),
        Severity::Info => format!("INFO: Port {} closed; no action required", port),
    };

    TriageResult {
        diff: diff.clone(),
        severity,
        action,
    }
}

pub fn triage_all(diffs: &[PortDiff]) -> Vec<TriageResult> {
    diffs.iter().map(triage).collect()
}

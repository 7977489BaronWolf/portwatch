use crate::severity::Severity;
use crate::severity_classifier::SeverityClassifier;

#[derive(Debug)]
pub struct SeverityCheckResult {
    pub port: u16,
    pub protocol: String,
    pub severity: Severity,
    pub actionable: bool,
    pub score: u8,
}

pub fn check_port_severity(
    classifier: &SeverityClassifier,
    port: u16,
    protocol: &str,
) -> SeverityCheckResult {
    let severity = classifier.classify(port, protocol);
    let actionable = severity.is_actionable();
    let score = severity.numeric_score();
    SeverityCheckResult {
        port,
        protocol: protocol.to_string(),
        severity,
        actionable,
        score,
    }
}

pub fn print_severity_report(results: &[SeverityCheckResult]) {
    println!("{:<8} {:<8} {:<10} {:<12} {}", "PORT", "PROTO", "SEVERITY", "ACTIONABLE", "SCORE");
    println!("{}", "-".repeat(52));
    for r in results {
        println!(
            "{:<8} {:<8} {:<10} {:<12} {}",
            r.port,
            r.protocol,
            r.severity,
            if r.actionable { "yes" } else { "no" },
            r.score
        );
    }
}

pub fn filter_actionable(results: Vec<SeverityCheckResult>) -> Vec<SeverityCheckResult> {
    results.into_iter().filter(|r| r.actionable).collect()
}

pub fn highest_severity(results: &[SeverityCheckResult]) -> Option<&Severity> {
    results.iter().map(|r| &r.severity).max()
}

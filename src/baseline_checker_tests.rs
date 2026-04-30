#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::baseline::Baseline;
    use crate::baseline_checker::BaselineChecker;
    use crate::port_scanner::PortInfo;

    fn port(p: u16) -> PortInfo {
        PortInfo { port: p, protocol: "tcp".to_string(), process: None }
    }

    #[test]
    fn test_no_violations() {
        let baseline = Baseline::new(HashSet::from([80, 443]), "test");
        let checker = BaselineChecker::default();
        let current = vec![port(80), port(443)];
        let result = checker.check(&baseline, &current);
        assert!(result.is_clean());
    }

    #[test]
    fn test_unexpected_port_detected() {
        let baseline = Baseline::new(HashSet::from([80, 443]), "test");
        let checker = BaselineChecker { alert_on_unexpected: true, alert_on_missing: false };
        let current = vec![port(80), port(443), port(4444)];
        let result = checker.check(&baseline, &current);
        assert!(!result.is_clean());
        assert!(result.unexpected.contains(&4444));
        assert!(result.missing.is_empty());
    }

    #[test]
    fn test_missing_port_detected_when_enabled() {
        let baseline = Baseline::new(HashSet::from([80, 443, 22]), "test");
        let checker = BaselineChecker { alert_on_unexpected: false, alert_on_missing: true };
        let current = vec![port(80), port(443)];
        let result = checker.check(&baseline, &current);
        assert!(!result.is_clean());
        assert!(result.missing.contains(&22));
        assert!(result.unexpected.is_empty());
    }

    #[test]
    fn test_missing_port_ignored_when_disabled() {
        let baseline = Baseline::new(HashSet::from([80, 443, 22]), "test");
        let checker = BaselineChecker::default();
        let current = vec![port(80), port(443)];
        let result = checker.check(&baseline, &current);
        assert!(result.is_clean());
    }

    #[test]
    fn test_summary_format() {
        let baseline = Baseline::new(HashSet::from([80]), "test");
        let checker = BaselineChecker { alert_on_unexpected: true, alert_on_missing: true };
        let current = vec![port(443)];
        let result = checker.check(&baseline, &current);
        let summary = result.summary();
        assert!(summary.contains("Unexpected") || summary.contains("Missing"));
    }
}

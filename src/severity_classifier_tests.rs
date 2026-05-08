#[cfg(test)]
mod tests {
    use super::super::severity::Severity;
    use super::super::severity_classifier::{SeverityClassifier, SeverityRule};

    fn make_classifier() -> SeverityClassifier {
        let mut c = SeverityClassifier::new();
        c.add_rule(SeverityRule {
            port_range: Some((1, 1023)),
            protocol: None,
            severity: Severity::High,
        });
        c.add_rule(SeverityRule {
            port_range: Some((1024, 49151)),
            protocol: Some("tcp".to_string()),
            severity: Severity::Medium,
        });
        c
    }

    #[test]
    fn test_classify_well_known_port() {
        let c = make_classifier();
        assert_eq!(c.classify(80, "tcp"), Severity::High);
        assert_eq!(c.classify(443, "tcp"), Severity::High);
        assert_eq!(c.classify(22, "tcp"), Severity::High);
    }

    #[test]
    fn test_classify_registered_port_tcp() {
        let c = make_classifier();
        assert_eq!(c.classify(8080, "tcp"), Severity::Medium);
    }

    #[test]
    fn test_classify_registered_port_udp_no_match() {
        let c = make_classifier();
        // UDP doesn't match the tcp rule, falls through to Info
        assert_eq!(c.classify(8080, "udp"), Severity::Info);
    }

    #[test]
    fn test_port_override_takes_precedence() {
        let mut c = make_classifier();
        c.set_port_override(80, Severity::Critical);
        assert_eq!(c.classify(80, "tcp"), Severity::Critical);
    }

    #[test]
    fn test_classify_unknown_port_defaults_info() {
        let c = make_classifier();
        assert_eq!(c.classify(60000, "tcp"), Severity::Info);
    }

    #[test]
    fn test_classify_new_port_system() {
        assert_eq!(SeverityClassifier::classify_new_port(80), Severity::High);
        assert_eq!(SeverityClassifier::classify_new_port(8080), Severity::Medium);
        assert_eq!(SeverityClassifier::classify_new_port(55000), Severity::Low);
    }

    #[test]
    fn test_empty_classifier_returns_info() {
        let c = SeverityClassifier::new();
        assert_eq!(c.classify(443, "tcp"), Severity::Info);
    }
}

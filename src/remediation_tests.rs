#[cfg(test)]
mod tests {
    use crate::diff_engine::PortDiff;
    use crate::remediation::{RemediationAction, RemediationEngine, RemediationRule};

    fn make_diff(opened: Vec<u16>, closed: Vec<u16>) -> PortDiff {
        PortDiff { opened, closed }
    }

    #[test]
    fn test_notify_only_default_action() {
        let engine = RemediationEngine::new(false);
        let diff = make_diff(vec![8080], vec![]);
        let results = engine.evaluate(&diff);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].action, RemediationAction::NotifyOnly);
        assert!(!results[0].executed);
    }

    #[test]
    fn test_custom_rule_applied() {
        let mut engine = RemediationEngine::new(false);
        engine.add_rule(RemediationRule {
            port: 4444,
            action: RemediationAction::BlockPort(4444),
        });
        let diff = make_diff(vec![4444], vec![]);
        let results = engine.evaluate(&diff);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].action, RemediationAction::BlockPort(4444));
        assert!(results[0].executed);
    }

    #[test]
    fn test_dry_run_prevents_execution() {
        let mut engine = RemediationEngine::new(true);
        engine.add_rule(RemediationRule {
            port: 9999,
            action: RemediationAction::KillProcess(1234),
        });
        let diff = make_diff(vec![9999], vec![]);
        let results = engine.evaluate(&diff);
        assert!(results[0].dry_run);
        assert!(!results[0].executed);
    }

    #[test]
    fn test_no_results_for_empty_diff() {
        let engine = RemediationEngine::new(false);
        let diff = make_diff(vec![], vec![22]);
        let results = engine.evaluate(&diff);
        assert!(results.is_empty());
    }

    #[test]
    fn test_summary_dry_run() {
        let mut engine = RemediationEngine::new(true);
        engine.add_rule(RemediationRule {
            port: 3000,
            action: RemediationAction::Custom("restart_service".to_string()),
        });
        let diff = make_diff(vec![3000], vec![]);
        let results = engine.evaluate(&diff);
        let summary = results[0].summary();
        assert!(summary.contains("DRY-RUN"));
        assert!(summary.contains("3000"));
    }

    #[test]
    fn test_multiple_ports_evaluated() {
        let engine = RemediationEngine::new(false);
        let diff = make_diff(vec![80, 443, 8443], vec![]);
        let results = engine.evaluate(&diff);
        assert_eq!(results.len(), 3);
    }
}

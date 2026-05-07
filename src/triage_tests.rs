#[cfg(test)]
mod tests {
    use crate::diff_engine::PortDiff;
    use crate::triage::{triage, triage_all, Severity};

    fn make_diff(port: u16, is_opened: bool) -> PortDiff {
        PortDiff { port, is_opened }
    }

    #[test]
    fn test_critical_port_opened() {
        let result = triage(&make_diff(22, true));
        assert_eq!(result.severity, Severity::Critical);
        assert!(result.action.contains("IMMEDIATE"));
    }

    #[test]
    fn test_critical_port_closed_is_medium() {
        let result = triage(&make_diff(22, false));
        assert_eq!(result.severity, Severity::Medium);
    }

    #[test]
    fn test_high_port_opened() {
        let result = triage(&make_diff(80, true));
        assert_eq!(result.severity, Severity::High);
        assert!(result.action.contains("REVIEW"));
    }

    #[test]
    fn test_privileged_non_listed_port_opened() {
        // Port 512 is privileged but not in CRITICAL or HIGH lists
        let result = triage(&make_diff(512, true));
        assert_eq!(result.severity, Severity::Medium);
        assert!(result.action.contains("MONITOR"));
    }

    #[test]
    fn test_unprivileged_port_opened() {
        let result = triage(&make_diff(49152, true));
        assert_eq!(result.severity, Severity::Low);
        assert!(result.action.contains("LOG"));
    }

    #[test]
    fn test_non_critical_port_closed_is_info() {
        let result = triage(&make_diff(8080, false));
        assert_eq!(result.severity, Severity::Info);
        assert!(result.action.contains("INFO"));
    }

    #[test]
    fn test_triage_all_returns_correct_count() {
        let diffs = vec![
            make_diff(22, true),
            make_diff(80, true),
            make_diff(9000, false),
        ];
        let results = triage_all(&diffs);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].severity, Severity::Critical);
        assert_eq!(results[1].severity, Severity::High);
        assert_eq!(results[2].severity, Severity::Info);
    }

    #[test]
    fn test_triage_all_empty() {
        let results = triage_all(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_port_1337_is_critical() {
        let result = triage(&make_diff(1337, true));
        assert_eq!(result.severity, Severity::Critical);
    }
}

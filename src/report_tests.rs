#[cfg(test)]
mod tests {
    use crate::diff_engine::PortDiff;
    use crate::report::Report;
    use crate::state_store::PortState;
    use std::collections::HashSet;

    fn make_state(ports: &[u16]) -> PortState {
        PortState {
            ports: ports.iter().cloned().collect::<HashSet<u16>>(),
            timestamp: chrono::Local::now(),
        }
    }

    fn make_diff_opened(port: u16) -> PortDiff {
        PortDiff::Opened(port)
    }

    fn make_diff_closed(port: u16) -> PortDiff {
        PortDiff::Closed(port)
    }

    #[test]
    fn test_report_no_changes() {
        let state = make_state(&[80, 443, 22]);
        let report = Report::new(state, vec![]);
        assert!(!report.has_changes());
        assert_eq!(report.total_open, 3);
        let summary = report.to_summary();
        assert!(summary.contains("No port changes"));
        assert!(summary.contains("3 ports open"));
    }

    #[test]
    fn test_report_with_changes() {
        let state = make_state(&[80, 443, 8080]);
        let diffs = vec![make_diff_opened(8080), make_diff_closed(22)];
        let report = Report::new(state, diffs);
        assert!(report.has_changes());
        assert_eq!(report.total_open, 3);
        let summary = report.to_summary();
        assert!(summary.contains("2 port change(s)"));
    }

    #[test]
    fn test_report_detailed_contains_diffs() {
        let state = make_state(&[80]);
        let diffs = vec![make_diff_opened(80)];
        let report = Report::new(state, diffs);
        let detailed = report.to_detailed();
        assert!(detailed.contains("Changes:"));
        assert!(detailed.contains("80"));
    }

    #[test]
    fn test_report_display_trait() {
        let state = make_state(&[443]);
        let report = Report::new(state, vec![]);
        let display = format!("{}", report);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_report_hostname_not_empty() {
        let state = make_state(&[]);
        let report = Report::new(state, vec![]);
        assert!(!report.hostname.is_empty());
    }
}

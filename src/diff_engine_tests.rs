#[cfg(test)]
mod diff_engine_tests {
    use crate::diff_engine::{compute_diff, format_diff_message, PortDiff};

    #[test]
    fn test_no_changes() {
        let prev = vec![80, 443, 8080];
        let curr = vec![80, 443, 8080];
        let diff = compute_diff(&prev, &curr);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_new_port_opened() {
        let prev = vec![80, 443];
        let curr = vec![80, 443, 8080];
        let diff = compute_diff(&prev, &curr);
        assert_eq!(diff.opened, vec![8080]);
        assert!(diff.closed.is_empty());
    }

    #[test]
    fn test_port_closed() {
        let prev = vec![80, 443, 8080];
        let curr = vec![80, 443];
        let diff = compute_diff(&prev, &curr);
        assert!(diff.opened.is_empty());
        assert_eq!(diff.closed, vec![8080]);
    }

    #[test]
    fn test_multiple_changes() {
        let prev = vec![80, 443, 22];
        let curr = vec![80, 8080, 9090];
        let diff = compute_diff(&prev, &curr);
        assert_eq!(diff.opened, vec![8080, 9090]);
        assert_eq!(diff.closed, vec![22, 443]);
    }

    #[test]
    fn test_from_empty_previous() {
        let prev: Vec<u16> = vec![];
        let curr = vec![80, 443];
        let diff = compute_diff(&prev, &curr);
        assert_eq!(diff.opened, vec![80, 443]);
        assert!(diff.closed.is_empty());
    }

    #[test]
    fn test_format_diff_message_no_changes() {
        let diff = PortDiff { opened: vec![], closed: vec![] };
        assert_eq!(format_diff_message(&diff), "No port changes detected.");
    }

    #[test]
    fn test_format_diff_message_opened_only() {
        let diff = PortDiff { opened: vec![8080], closed: vec![] };
        assert_eq!(format_diff_message(&diff), "Opened ports: 8080");
    }

    #[test]
    fn test_format_diff_message_both() {
        let diff = PortDiff { opened: vec![8080], closed: vec![22] };
        let msg = format_diff_message(&diff);
        assert!(msg.contains("Opened ports: 8080"));
        assert!(msg.contains("Closed ports: 22"));
    }
}

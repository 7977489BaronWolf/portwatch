#[cfg(test)]
mod tests {
    use super::super::aggregation_cmd::{AggregationCmd, AggregationCmdHandler};

    #[test]
    fn test_status_empty() {
        let handler = AggregationCmdHandler::new(60);
        // We can't call handle on immutable, but handler owns aggregator
        let mut handler = handler;
        let out = handler.handle(AggregationCmd::Status);
        assert!(out.contains("No active"));
    }

    #[test]
    fn test_flush_empty() {
        let mut handler = AggregationCmdHandler::new(60);
        let out = handler.handle(AggregationCmd::Flush);
        assert!(out.contains("0 aggregation group"));
    }

    #[test]
    fn test_status_after_submit() {
        let mut handler = AggregationCmdHandler::new(60);
        handler.aggregator.submit("port:8080", "opened");
        handler.aggregator.submit("port:8080", "again");
        let out = handler.handle(AggregationCmd::Status);
        assert!(out.contains("port:8080"));
        assert!(out.contains("count=2"));
    }

    #[test]
    fn test_flush_clears_via_cmd() {
        let mut handler = AggregationCmdHandler::new(60);
        handler.aggregator.submit("port:22", "ssh");
        let out = handler.handle(AggregationCmd::Flush);
        assert!(out.contains("1 aggregation group"));
        let out2 = handler.handle(AggregationCmd::Status);
        assert!(out2.contains("No active"));
    }

    #[test]
    fn test_set_window_resets_state() {
        let mut handler = AggregationCmdHandler::new(60);
        handler.aggregator.submit("port:443", "https");
        let out = handler.handle(AggregationCmd::SetWindow(30));
        assert!(out.contains("30s"));
        assert_eq!(handler.window_secs, 30);
        // State should be cleared
        assert_eq!(handler.aggregator.group_count(), 0);
    }

    #[test]
    fn test_multiple_groups_in_status() {
        let mut handler = AggregationCmdHandler::new(60);
        handler.aggregator.submit("port:80", "http");
        handler.aggregator.submit("port:443", "https");
        let out = handler.handle(AggregationCmd::Status);
        assert!(out.contains("Active groups: 2"));
        assert!(out.contains("port:80"));
        assert!(out.contains("port:443"));
    }
}

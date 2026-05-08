#[cfg(test)]
mod tests {
    use super::super::rollup::*;
    use std::time::Duration;

    #[test]
    fn test_empty_flush_returns_empty_summary() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        let summary = buf.flush();
        assert!(summary.is_empty());
        assert_eq!(summary.event_count, 0);
    }

    #[test]
    fn test_single_opened_event() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        buf.push(PortEvent::new(8080, ChangeKind::Opened));
        let summary = buf.flush();
        assert_eq!(summary.opened, vec![8080]);
        assert!(summary.closed.is_empty());
        assert_eq!(summary.event_count, 1);
    }

    #[test]
    fn test_single_closed_event() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        buf.push(PortEvent::new(443, ChangeKind::Closed));
        let summary = buf.flush();
        assert!(summary.opened.is_empty());
        assert_eq!(summary.closed, vec![443]);
    }

    #[test]
    fn test_multiple_events_deduped_by_port() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        buf.push(PortEvent::new(80, ChangeKind::Opened));
        buf.push(PortEvent::new(80, ChangeKind::Closed)); // last write wins
        buf.push(PortEvent::new(443, ChangeKind::Opened));
        let summary = buf.flush();
        // port 80 last seen as Closed
        assert!(summary.closed.contains(&80));
        assert!(summary.opened.contains(&443));
    }

    #[test]
    fn test_flush_clears_buffer() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        buf.push(PortEvent::new(22, ChangeKind::Opened));
        let _ = buf.flush();
        let summary2 = buf.flush();
        assert!(summary2.is_empty());
    }

    #[test]
    fn test_summary_format_contains_keywords() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        buf.push(PortEvent::new(3000, ChangeKind::Opened));
        let summary = buf.flush();
        let text = summary.format();
        assert!(text.contains("Rollup"));
        assert!(text.contains("opened"));
        assert!(text.contains("closed"));
    }

    #[test]
    fn test_opened_and_closed_ports_sorted() {
        let mut buf = RollupBuffer::new(Duration::from_secs(60));
        for port in [9000u16, 1000, 5000] {
            buf.push(PortEvent::new(port, ChangeKind::Opened));
        }
        let summary = buf.flush();
        assert_eq!(summary.opened, vec![1000, 5000, 9000]);
    }
}

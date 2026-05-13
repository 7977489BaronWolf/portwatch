#[cfg(test)]
mod tests {
    use super::super::window::{TimeWindow, WindowEvent};
    use super::super::window_aggregator::WindowAggregator;
    use std::time::{Duration, Instant};

    fn ev(port: u16, kind: &str) -> WindowEvent {
        WindowEvent { port, kind: kind.to_string(), timestamp: Instant::now() }
    }

    #[test]
    fn test_summarize_empty() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        let s = WindowAggregator::summarize(&mut w);
        assert_eq!(s.total, 0);
        assert!(s.top_port.is_none());
    }

    #[test]
    fn test_summarize_counts() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(ev(80, "open"));
        w.push(ev(80, "open"));
        w.push(ev(22, "close"));
        let s = WindowAggregator::summarize(&mut w);
        assert_eq!(s.total, 3);
        assert_eq!(s.by_kind.get("open"), Some(&2));
        assert_eq!(s.by_kind.get("close"), Some(&1));
        assert_eq!(s.by_port.get(&80), Some(&2));
    }

    #[test]
    fn test_summarize_top_port() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(ev(443, "open"));
        w.push(ev(443, "open"));
        w.push(ev(443, "open"));
        w.push(ev(80, "open"));
        let s = WindowAggregator::summarize(&mut w);
        assert_eq!(s.top_port, Some(443));
    }

    #[test]
    fn test_hot_ports_none_below_threshold() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(ev(8080, "open"));
        w.push(ev(9090, "open"));
        let hot = WindowAggregator::hot_ports(&mut w, 3);
        assert!(hot.is_empty());
    }

    #[test]
    fn test_hot_ports_detected() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        for _ in 0..5 {
            w.push(ev(6379, "open"));
        }
        w.push(ev(80, "open"));
        let hot = WindowAggregator::hot_ports(&mut w, 3);
        assert_eq!(hot, vec![6379]);
    }

    #[test]
    fn test_hot_ports_multiple() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        for _ in 0..4 { w.push(ev(22, "open")); }
        for _ in 0..4 { w.push(ev(443, "open")); }
        let hot = WindowAggregator::hot_ports(&mut w, 4);
        assert_eq!(hot.len(), 2);
        assert!(hot.contains(&22));
        assert!(hot.contains(&443));
    }

    #[test]
    fn test_summarize_does_not_drain() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(ev(80, "open"));
        let _ = WindowAggregator::summarize(&mut w);
        assert_eq!(w.count(), 1);
    }
}

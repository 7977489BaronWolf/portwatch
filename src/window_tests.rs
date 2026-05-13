#[cfg(test)]
mod tests {
    use super::super::window::{TimeWindow, WindowEvent};
    use std::time::{Duration, Instant};

    fn make_event(port: u16, kind: &str) -> WindowEvent {
        WindowEvent {
            port,
            kind: kind.to_string(),
            timestamp: Instant::now(),
        }
    }

    fn make_old_event(port: u16, kind: &str, age_secs: u64) -> WindowEvent {
        WindowEvent {
            port,
            kind: kind.to_string(),
            timestamp: Instant::now() - Duration::from_secs(age_secs),
        }
    }

    #[test]
    fn test_push_and_count() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        assert_eq!(w.count(), 0);
        w.push(make_event(80, "open"));
        w.push(make_event(443, "open"));
        assert_eq!(w.count(), 2);
    }

    #[test]
    fn test_eviction_of_old_events() {
        let mut w = TimeWindow::new(Duration::from_secs(30));
        w.push(make_old_event(8080, "close", 60));
        w.push(make_event(9090, "open"));
        assert_eq!(w.count(), 1);
    }

    #[test]
    fn test_count_by_kind() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(make_event(80, "open"));
        w.push(make_event(443, "open"));
        w.push(make_event(22, "close"));
        assert_eq!(w.count_by_kind("open"), 2);
        assert_eq!(w.count_by_kind("close"), 1);
        assert_eq!(w.count_by_kind("unknown"), 0);
    }

    #[test]
    fn test_drain_clears_window() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(make_event(80, "open"));
        w.push(make_event(22, "open"));
        let drained = w.drain();
        assert_eq!(drained.len(), 2);
        assert!(w.is_empty());
    }

    #[test]
    fn test_events_returns_references() {
        let mut w = TimeWindow::new(Duration::from_secs(60));
        w.push(make_event(3000, "open"));
        let events = w.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port, 3000);
    }

    #[test]
    fn test_is_empty_on_new_window() {
        let mut w = TimeWindow::new(Duration::from_secs(10));
        assert!(w.is_empty());
    }

    #[test]
    fn test_all_old_events_evicted() {
        let mut w = TimeWindow::new(Duration::from_secs(5));
        w.push(make_old_event(80, "open", 10));
        w.push(make_old_event(443, "open", 20));
        assert!(w.is_empty());
    }
}

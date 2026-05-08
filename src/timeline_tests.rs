#[cfg(test)]
mod tests {
    use super::super::timeline::*;

    fn make_event(port: u16, kind: TimelineEventKind) -> TimelineEvent {
        TimelineEvent::new(port, "tcp", kind, "test event")
    }

    #[test]
    fn test_push_and_len() {
        let mut tl = Timeline::new();
        assert!(tl.is_empty());
        tl.push(make_event(80, TimelineEventKind::PortOpened));
        tl.push(make_event(443, TimelineEventKind::PortOpened));
        assert_eq!(tl.len(), 2);
    }

    #[test]
    fn test_capacity_eviction() {
        let mut tl = Timeline::with_capacity(3);
        tl.push(make_event(1, TimelineEventKind::PortOpened));
        tl.push(make_event(2, TimelineEventKind::PortOpened));
        tl.push(make_event(3, TimelineEventKind::PortOpened));
        tl.push(make_event(4, TimelineEventKind::PortOpened));
        assert_eq!(tl.len(), 3);
        // oldest (port 1) should have been evicted
        let ports: Vec<u16> = tl.events().iter().map(|e| e.port).collect();
        assert!(!ports.contains(&1));
        assert!(ports.contains(&4));
    }

    #[test]
    fn test_events_for_port() {
        let mut tl = Timeline::new();
        tl.push(make_event(80, TimelineEventKind::PortOpened));
        tl.push(make_event(443, TimelineEventKind::PortClosed));
        tl.push(make_event(80, TimelineEventKind::PortChanged));
        let port_80 = tl.events_for_port(80);
        assert_eq!(port_80.len(), 2);
        assert!(port_80.iter().all(|e| e.port == 80));
    }

    #[test]
    fn test_since_filter() {
        let mut tl = Timeline::new();
        let mut e1 = make_event(22, TimelineEventKind::PortOpened);
        e1.timestamp = 1000;
        let mut e2 = make_event(80, TimelineEventKind::PortOpened);
        e2.timestamp = 2000;
        let mut e3 = make_event(443, TimelineEventKind::PortOpened);
        e3.timestamp = 3000;
        tl.push(e1);
        tl.push(e2);
        tl.push(e3);
        let recent = tl.since(2000);
        assert_eq!(recent.len(), 2);
        assert!(recent.iter().all(|e| e.timestamp >= 2000));
    }

    #[test]
    fn test_clear() {
        let mut tl = Timeline::new();
        tl.push(make_event(80, TimelineEventKind::PortOpened));
        tl.clear();
        assert!(tl.is_empty());
    }

    #[test]
    fn test_event_kind_equality() {
        assert_eq!(TimelineEventKind::PortOpened, TimelineEventKind::PortOpened);
        assert_ne!(TimelineEventKind::PortOpened, TimelineEventKind::PortClosed);
    }

    #[test]
    fn test_event_fields() {
        let e = TimelineEvent::new(8080, "udp", TimelineEventKind::PortChanged, "changed pid");
        assert_eq!(e.port, 8080);
        assert_eq!(e.protocol, "udp");
        assert_eq!(e.description, "changed pid");
        assert!(e.timestamp > 0);
    }
}

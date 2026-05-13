#[cfg(test)]
mod tests {
    use crate::replay::{ReplayBuffer, ReplayEvent, ReplaySource};

    fn make_event(port: u16, source: ReplaySource, ts: u64) -> ReplayEvent {
        ReplayEvent {
            timestamp: ts,
            source,
            description: format!("event on port {}", port),
            port,
            protocol: "tcp".to_string(),
        }
    }

    #[test]
    fn test_push_and_len() {
        let mut buf = ReplayBuffer::new();
        assert_eq!(buf.len(), 0);
        buf.push(make_event(80, ReplaySource::AuditLog, 1000));
        buf.push(make_event(443, ReplaySource::Snapshot, 2000));
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_next_fifo_order() {
        let mut buf = ReplayBuffer::new();
        buf.push(make_event(80, ReplaySource::AuditLog, 1000));
        buf.push(make_event(443, ReplaySource::AuditLog, 2000));
        let first = buf.next().unwrap();
        assert_eq!(first.port, 80);
        let second = buf.next().unwrap();
        assert_eq!(second.port, 443);
        assert!(buf.next().is_none());
    }

    #[test]
    fn test_is_empty() {
        let mut buf = ReplayBuffer::new();
        assert!(buf.is_empty());
        buf.push(make_event(22, ReplaySource::AuditLog, 500));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_drain_all() {
        let mut buf = ReplayBuffer::new();
        buf.push(make_event(80, ReplaySource::AuditLog, 100));
        buf.push(make_event(8080, ReplaySource::Snapshot, 200));
        let drained = buf.drain_all();
        assert_eq!(drained.len(), 2);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_filter_by_port() {
        let mut buf = ReplayBuffer::new();
        buf.push(make_event(80, ReplaySource::AuditLog, 100));
        buf.push(make_event(443, ReplaySource::AuditLog, 200));
        buf.push(make_event(80, ReplaySource::Snapshot, 300));
        let results = buf.filter_by_port(80);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| e.port == 80));
    }

    #[test]
    fn test_filter_by_source() {
        let mut buf = ReplayBuffer::new();
        buf.push(make_event(80, ReplaySource::AuditLog, 100));
        buf.push(make_event(443, ReplaySource::Snapshot, 200));
        buf.push(make_event(22, ReplaySource::AuditLog, 300));
        let audit_events = buf.filter_by_source(&ReplaySource::AuditLog);
        assert_eq!(audit_events.len(), 2);
        let snap_events = buf.filter_by_source(&ReplaySource::Snapshot);
        assert_eq!(snap_events.len(), 1);
        assert_eq!(snap_events[0].port, 443);
    }

    #[test]
    fn test_event_description_snapshot() {
        let event = make_event(8443, ReplaySource::Snapshot, 9999);
        assert!(event.description.contains("8443"));
        assert_eq!(event.source, ReplaySource::Snapshot);
    }
}

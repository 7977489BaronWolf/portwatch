#[cfg(test)]
mod tests {
    use crate::diff_engine::{PortChange, ChangeKind};
    use crate::grouping::{group_by_protocol, group_by_time_window, GroupKey};

    fn make_change(port: u16, protocol: &str, timestamp: u64) -> PortChange {
        PortChange {
            port,
            protocol: protocol.to_string(),
            kind: ChangeKind::Opened,
            timestamp,
        }
    }

    #[test]
    fn test_group_by_protocol_single_proto() {
        let changes = vec![
            make_change(80, "tcp", 1000),
            make_change(443, "tcp", 2000),
        ];
        let groups = group_by_protocol(&changes);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[0].key, GroupKey::Protocol("tcp".to_string()));
    }

    #[test]
    fn test_group_by_protocol_multiple_protos() {
        let changes = vec![
            make_change(80, "tcp", 1000),
            make_change(53, "udp", 1500),
            make_change(443, "tcp", 2000),
            make_change(123, "udp", 2500),
        ];
        let mut groups = group_by_protocol(&changes);
        groups.sort_by_key(|g| match &g.key {
            GroupKey::Protocol(p) => p.clone(),
            _ => String::new(),
        });
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].key, GroupKey::Protocol("tcp".to_string()));
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[1].key, GroupKey::Protocol("udp".to_string()));
        assert_eq!(groups[1].len(), 2);
    }

    #[test]
    fn test_group_by_protocol_empty() {
        let groups = group_by_protocol(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_by_time_window_basic() {
        let changes = vec![
            make_change(80, "tcp", 0),
            make_change(443, "tcp", 30),
            make_change(8080, "tcp", 61),
        ];
        let groups = group_by_time_window(&changes, 60);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].key, GroupKey::TimeWindow(0));
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[1].key, GroupKey::TimeWindow(60));
        assert_eq!(groups[1].len(), 1);
    }

    #[test]
    fn test_group_by_time_window_zero_bucket_returns_empty() {
        let changes = vec![make_change(80, "tcp", 100)];
        let groups = group_by_time_window(&changes, 0);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_by_time_window_all_same_bucket() {
        let changes = vec![
            make_change(80, "tcp", 10),
            make_change(443, "tcp", 20),
            make_change(22, "tcp", 59),
        ];
        let groups = group_by_time_window(&changes, 60);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 3);
    }
}

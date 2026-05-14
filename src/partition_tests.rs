#[cfg(test)]
mod tests {
    use crate::partition::{PartitionKey, PartitionRule, Partitioner};
    use crate::port_scanner::PortEntry;

    fn make_entry(port: u16, protocol: &str) -> PortEntry {
        PortEntry {
            port,
            protocol: protocol.to_string(),
            state: "LISTEN".to_string(),
            process: None,
        }
    }

    #[test]
    fn test_protocol_partition() {
        let mut p = Partitioner::new();
        p.add_rule(PartitionRule::new(
            PartitionKey::Protocol("tcp".to_string()),
            "TCP ports",
        ));
        let entries = vec![
            make_entry(80, "tcp"),
            make_entry(53, "udp"),
            make_entry(443, "TCP"),
        ];
        let buckets = p.partition(&entries);
        assert_eq!(buckets["proto:tcp"].len(), 2);
        assert_eq!(buckets["unclassified"].len(), 1);
    }

    #[test]
    fn test_port_range_partition() {
        let mut p = Partitioner::new();
        p.add_rule(PartitionRule::new(
            PartitionKey::PortRange(1, 1024),
            "Well-known ports",
        ));
        p.add_rule(PartitionRule::new(
            PartitionKey::PortRange(1025, 49151),
            "Registered ports",
        ));
        let entries = vec![
            make_entry(22, "tcp"),
            make_entry(8080, "tcp"),
            make_entry(60000, "tcp"),
        ];
        let buckets = p.partition(&entries);
        assert_eq!(buckets["range:1-1024"].len(), 1);
        assert_eq!(buckets["range:1025-49151"].len(), 1);
        assert_eq!(buckets["unclassified"].len(), 1);
    }

    #[test]
    fn test_all_unclassified_when_no_rules() {
        let p = Partitioner::new();
        let entries = vec![make_entry(80, "tcp"), make_entry(443, "tcp")];
        let buckets = p.partition(&entries);
        assert_eq!(buckets["unclassified"].len(), 2);
        assert_eq!(p.rule_count(), 0);
    }

    #[test]
    fn test_empty_input() {
        let mut p = Partitioner::new();
        p.add_rule(PartitionRule::new(
            PartitionKey::Protocol("tcp".to_string()),
            "TCP",
        ));
        let buckets = p.partition(&[]);
        assert!(buckets.is_empty());
    }

    #[test]
    fn test_partition_key_label() {
        assert_eq!(PartitionKey::Protocol("udp".to_string()).label(), "proto:udp");
        assert_eq!(PartitionKey::PortRange(80, 443).label(), "range:80-443");
        assert_eq!(PartitionKey::Custom("foo".to_string()).label(), "custom:foo");
    }

    #[test]
    fn test_first_matching_rule_wins() {
        let mut p = Partitioner::new();
        p.add_rule(PartitionRule::new(
            PartitionKey::PortRange(1, 65535),
            "all",
        ));
        p.add_rule(PartitionRule::new(
            PartitionKey::Protocol("tcp".to_string()),
            "tcp",
        ));
        let entries = vec![make_entry(80, "tcp")];
        let buckets = p.partition(&entries);
        assert!(buckets.contains_key("range:1-65535"));
        assert!(!buckets.contains_key("proto:tcp"));
    }
}

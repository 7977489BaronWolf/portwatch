#[cfg(test)]
mod tests {
    use chrono::Duration;
    use crate::suppression_rule::{
        SuppressionCondition, SuppressionRule, SuppressionRuleStore,
    };

    fn make_rule(id: &str) -> SuppressionRule {
        SuppressionRule::new(id, "Test Rule", "maintenance")
    }

    #[test]
    fn test_exact_port_match() {
        let rule = make_rule("r1")
            .with_condition(SuppressionCondition::ExactPort(8080));
        assert!(rule.matches_port(8080, "tcp", &[]));
        assert!(!rule.matches_port(9090, "tcp", &[]));
    }

    #[test]
    fn test_port_range_match() {
        let rule = make_rule("r2")
            .with_condition(SuppressionCondition::PortRange(8000, 8999));
        assert!(rule.matches_port(8080, "tcp", &[]));
        assert!(rule.matches_port(8000, "tcp", &[]));
        assert!(rule.matches_port(8999, "tcp", &[]));
        assert!(!rule.matches_port(7999, "tcp", &[]));
        assert!(!rule.matches_port(9000, "tcp", &[]));
    }

    #[test]
    fn test_protocol_match() {
        let rule = make_rule("r3")
            .with_condition(SuppressionCondition::Protocol("udp".into()));
        assert!(rule.matches_port(53, "udp", &[]));
        assert!(rule.matches_port(53, "UDP", &[]));
        assert!(!rule.matches_port(53, "tcp", &[]));
    }

    #[test]
    fn test_tag_match() {
        let rule = make_rule("r4")
            .with_condition(SuppressionCondition::Tag("internal".into()));
        let tags = vec!["internal".to_string(), "monitored".to_string()];
        assert!(rule.matches_port(443, "tcp", &tags));
        assert!(!rule.matches_port(443, "tcp", &["external".to_string()]));
    }

    #[test]
    fn test_expired_rule_does_not_match() {
        let rule = make_rule("r5")
            .with_condition(SuppressionCondition::ExactPort(22))
            .with_ttl(Duration::seconds(-1));
        assert!(rule.is_expired());
        assert!(!rule.matches_port(22, "tcp", &[]));
    }

    #[test]
    fn test_empty_conditions_matches_all() {
        let rule = make_rule("r6");
        assert!(rule.matches_port(1234, "tcp", &[]));
    }

    #[test]
    fn test_store_add_and_suppress() {
        let mut store = SuppressionRuleStore::new();
        let rule = make_rule("s1")
            .with_condition(SuppressionCondition::ExactPort(3306));
        store.add(rule);
        assert!(store.is_suppressed(3306, "tcp", &[]));
        assert!(!store.is_suppressed(5432, "tcp", &[]));
    }

    #[test]
    fn test_store_remove() {
        let mut store = SuppressionRuleStore::new();
        store.add(make_rule("s2").with_condition(SuppressionCondition::ExactPort(80)));
        assert!(store.remove("s2").is_some());
        assert!(!store.is_suppressed(80, "tcp", &[]));
    }

    #[test]
    fn test_purge_expired() {
        let mut store = SuppressionRuleStore::new();
        store.add(
            make_rule("e1")
                .with_condition(SuppressionCondition::ExactPort(9000))
                .with_ttl(Duration::seconds(-10)),
        );
        store.add(make_rule("e2").with_condition(SuppressionCondition::ExactPort(9001)));
        let purged = store.purge_expired();
        assert_eq!(purged, 1);
        assert_eq!(store.list_active().len(), 1);
    }
}

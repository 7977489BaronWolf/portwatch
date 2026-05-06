#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::suppress::{SuppressionRule, SuppressionStore};

    fn short_rule(port: u16, proto: &str) -> SuppressionRule {
        SuppressionRule::new(port, proto, "maintenance", Duration::from_secs(60))
    }

    fn expired_rule(port: u16, proto: &str) -> SuppressionRule {
        SuppressionRule::new(port, proto, "expired", Duration::from_millis(0))
    }

    #[test]
    fn test_add_and_is_suppressed() {
        let mut store = SuppressionStore::new();
        store.add(short_rule(8080, "tcp"));
        assert!(store.is_suppressed(8080, "tcp"));
        assert!(!store.is_suppressed(8080, "udp"));
        assert!(!store.is_suppressed(9090, "tcp"));
    }

    #[test]
    fn test_expired_rule_not_suppressed() {
        let mut store = SuppressionStore::new();
        store.add(expired_rule(443, "tcp"));
        // zero-duration rule should already be expired
        assert!(!store.is_suppressed(443, "tcp"));
    }

    #[test]
    fn test_remove_rule() {
        let mut store = SuppressionStore::new();
        store.add(short_rule(22, "tcp"));
        assert!(store.is_suppressed(22, "tcp"));
        store.remove(22, "tcp");
        assert!(!store.is_suppressed(22, "tcp"));
    }

    #[test]
    fn test_purge_expired_removes_only_expired() {
        let mut store = SuppressionStore::new();
        store.add(short_rule(80, "tcp"));
        store.add(expired_rule(9999, "udp"));
        assert_eq!(store.len(), 2);
        let removed = store.purge_expired();
        assert_eq!(removed, 1);
        assert_eq!(store.len(), 1);
        assert!(store.is_suppressed(80, "tcp"));
    }

    #[test]
    fn test_active_rules_excludes_expired() {
        let mut store = SuppressionStore::new();
        store.add(short_rule(3306, "tcp"));
        store.add(expired_rule(5432, "tcp"));
        let active = store.active_rules();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].port, 3306);
    }

    #[test]
    fn test_overwrite_existing_rule() {
        let mut store = SuppressionStore::new();
        store.add(short_rule(8443, "tcp"));
        store.add(SuppressionRule::new(8443, "tcp", "updated", Duration::from_secs(120)));
        assert_eq!(store.len(), 1);
        assert!(store.is_suppressed(8443, "tcp"));
    }

    #[test]
    fn test_is_empty_and_len() {
        let mut store = SuppressionStore::new();
        assert!(store.is_empty());
        store.add(short_rule(53, "udp"));
        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::digest::{compute_digest, has_changed, DigestCache};

    fn ports(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn same_ports_produce_same_digest() {
        let a = compute_digest(&ports(&["tcp:80", "tcp:443"]));
        let b = compute_digest(&ports(&["tcp:80", "tcp:443"]));
        assert_eq!(a, b);
    }

    #[test]
    fn order_independent_digest() {
        let a = compute_digest(&ports(&["tcp:80", "udp:53", "tcp:443"]));
        let b = compute_digest(&ports(&["tcp:443", "tcp:80", "udp:53"]));
        assert_eq!(a, b, "digest must be order-independent");
    }

    #[test]
    fn different_ports_produce_different_digest() {
        let a = compute_digest(&ports(&["tcp:80"]));
        let b = compute_digest(&ports(&["tcp:8080"]));
        assert_ne!(a, b);
    }

    #[test]
    fn empty_port_list_is_stable() {
        let a = compute_digest(&[]);
        let b = compute_digest(&[]);
        assert_eq!(a, b);
    }

    #[test]
    fn has_changed_detects_difference() {
        let a = compute_digest(&ports(&["tcp:22"]));
        let b = compute_digest(&ports(&["tcp:22", "tcp:3306"]));
        assert!(has_changed(&a, &b));
    }

    #[test]
    fn has_changed_returns_false_for_equal_digests() {
        let a = compute_digest(&ports(&["tcp:22"]));
        let b = compute_digest(&ports(&["tcp:22"]));
        assert!(!has_changed(&a, &b));
    }

    #[test]
    fn digest_cache_reports_changed_on_first_call() {
        let mut cache = DigestCache::new();
        let d = compute_digest(&ports(&["tcp:80"]));
        assert!(cache.update(d), "first update should always report changed");
    }

    #[test]
    fn digest_cache_reports_unchanged_when_state_stable() {
        let mut cache = DigestCache::new();
        let d = compute_digest(&ports(&["tcp:80"]));
        cache.update(d.clone());
        assert!(!cache.update(d), "second identical update should not report changed");
    }

    #[test]
    fn digest_cache_reports_changed_after_state_changes() {
        let mut cache = DigestCache::new();
        let d1 = compute_digest(&ports(&["tcp:80"]));
        let d2 = compute_digest(&ports(&["tcp:80", "tcp:9090"]));
        cache.update(d1);
        assert!(cache.update(d2), "should detect new port added");
    }

    #[test]
    fn digest_display_is_hex_string() {
        let d = compute_digest(&ports(&["tcp:443"]));
        assert!(d.to_string().chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(d.to_string().len(), 64);
    }
}

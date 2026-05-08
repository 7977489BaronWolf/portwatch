#[cfg(test)]
mod tests {
    use crate::tag::TagStore;

    fn store_with_data() -> TagStore {
        let mut s = TagStore::new();
        s.add_tag(80, "http");
        s.add_tag(80, "web");
        s.add_tag(443, "https");
        s.add_tag(443, "web");
        s.add_tag(22, "ssh");
        s
    }

    #[test]
    fn test_add_and_get_tags() {
        let s = store_with_data();
        let tags = s.get_tags(80);
        assert_eq!(tags, vec!["http", "web"]);
    }

    #[test]
    fn test_add_tag_returns_true_on_new() {
        let mut s = TagStore::new();
        assert!(s.add_tag(8080, "dev"));
    }

    #[test]
    fn test_add_tag_returns_false_on_duplicate() {
        let mut s = TagStore::new();
        s.add_tag(8080, "dev");
        assert!(!s.add_tag(8080, "dev"));
    }

    #[test]
    fn test_remove_tag() {
        let mut s = store_with_data();
        assert!(s.remove_tag(80, "http"));
        assert_eq!(s.get_tags(80), vec!["web"]);
    }

    #[test]
    fn test_remove_last_tag_cleans_entry() {
        let mut s = TagStore::new();
        s.add_tag(9000, "only");
        s.remove_tag(9000, "only");
        assert!(s.get_tags(9000).is_empty());
    }

    #[test]
    fn test_remove_nonexistent_tag_returns_false() {
        let mut s = store_with_data();
        assert!(!s.remove_tag(80, "nonexistent"));
        assert!(!s.remove_tag(9999, "anything"));
    }

    #[test]
    fn test_ports_with_tag() {
        let s = store_with_data();
        assert_eq!(s.ports_with_tag("web"), vec![80, 443]);
        assert_eq!(s.ports_with_tag("ssh"), vec![22]);
        assert!(s.ports_with_tag("ftp").is_empty());
    }

    #[test]
    fn test_clear_port() {
        let mut s = store_with_data();
        s.clear_port(443);
        assert!(s.get_tags(443).is_empty());
        // Other ports unaffected
        assert!(!s.get_tags(80).is_empty());
    }

    #[test]
    fn test_snapshot_sorted() {
        let s = store_with_data();
        let snap = s.snapshot();
        let web_tags = snap.get(&80).unwrap();
        assert_eq!(web_tags, &vec!["http", "web"]);
    }

    #[test]
    fn test_default_is_empty() {
        let s: TagStore = Default::default();
        assert!(s.snapshot().is_empty());
    }
}

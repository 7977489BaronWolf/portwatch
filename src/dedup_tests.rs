#[cfg(test)]
mod tests {
    use super::super::dedup::DedupCache;
    use std::time::Duration;

    #[test]
    fn first_occurrence_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        assert!(!cache.is_duplicate("port:8080:opened"));
    }

    #[test]
    fn immediate_repeat_is_duplicate() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        cache.is_duplicate("port:8080:opened");
        assert!(cache.is_duplicate("port:8080:opened"));
    }

    #[test]
    fn different_keys_are_independent() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        assert!(!cache.is_duplicate("port:8080:opened"));
        assert!(!cache.is_duplicate("port:9090:opened"));
    }

    #[test]
    fn expired_entry_is_not_duplicate() {
        let mut cache = DedupCache::new(Duration::from_millis(10));
        cache.is_duplicate("port:8080:opened");
        std::thread::sleep(Duration::from_millis(20));
        assert!(!cache.is_duplicate("port:8080:opened"));
    }

    #[test]
    fn evict_expired_removes_stale_entries() {
        let mut cache = DedupCache::new(Duration::from_millis(10));
        cache.is_duplicate("key1");
        cache.is_duplicate("key2");
        assert_eq!(cache.len(), 2);
        std::thread::sleep(Duration::from_millis(20));
        cache.evict_expired();
        assert!(cache.is_empty());
    }

    #[test]
    fn evict_expired_keeps_fresh_entries() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        cache.is_duplicate("fresh");
        cache.evict_expired();
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn len_reflects_unique_keys() {
        let mut cache = DedupCache::new(Duration::from_secs(60));
        cache.is_duplicate("a");
        cache.is_duplicate("b");
        cache.is_duplicate("a"); // duplicate — should not increase count
        assert_eq!(cache.len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use super::super::cache::Cache;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_insert_and_get() {
        let mut cache: Cache<&str, u32> = Cache::new(Duration::from_secs(60));
        cache.insert("port_80", 80);
        assert_eq!(cache.get(&"port_80"), Some(&80));
    }

    #[test]
    fn test_get_missing_key_returns_none() {
        let cache: Cache<&str, u32> = Cache::new(Duration::from_secs(60));
        assert_eq!(cache.get(&"port_443"), None);
    }

    #[test]
    fn test_entry_expires_after_ttl() {
        let mut cache: Cache<&str, u32> = Cache::new(Duration::from_millis(50));
        cache.insert("port_22", 22);
        assert_eq!(cache.get(&"port_22"), Some(&22));
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get(&"port_22"), None);
    }

    #[test]
    fn test_insert_with_custom_ttl() {
        let mut cache: Cache<&str, &str> = Cache::new(Duration::from_secs(60));
        cache.insert_with_ttl("key", "value", Duration::from_millis(30));
        assert_eq!(cache.get(&"key"), Some(&"value"));
        thread::sleep(Duration::from_millis(40));
        assert_eq!(cache.get(&"key"), None);
    }

    #[test]
    fn test_remove_entry() {
        let mut cache: Cache<&str, i32> = Cache::new(Duration::from_secs(60));
        cache.insert("x", 42);
        assert_eq!(cache.remove(&"x"), Some(42));
        assert_eq!(cache.get(&"x"), None);
    }

    #[test]
    fn test_evict_expired_removes_stale_entries() {
        let mut cache: Cache<u16, &str> = Cache::new(Duration::from_millis(30));
        cache.insert(80, "http");
        cache.insert(443, "https");
        cache.insert_with_ttl(22, "ssh", Duration::from_secs(60));
        thread::sleep(Duration::from_millis(40));
        let evicted = cache.evict_expired();
        assert_eq!(evicted, 2);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&22), Some(&"ssh"));
    }

    #[test]
    fn test_clear_empties_cache() {
        let mut cache: Cache<u8, u8> = Cache::new(Duration::from_secs(60));
        cache.insert(1, 1);
        cache.insert(2, 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut cache: Cache<&str, bool> = Cache::new(Duration::from_secs(60));
        assert!(cache.is_empty());
        cache.insert("a", true);
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }
}

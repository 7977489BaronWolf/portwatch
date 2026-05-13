#[cfg(test)]
mod tests {
    use super::super::aggregation::Aggregator;
    use std::time::Duration;

    #[test]
    fn test_first_submission_returns_group() {
        let mut agg = Aggregator::new(Duration::from_secs(60));
        let result = agg.submit("port:8080", "new port opened");
        assert!(result.is_some());
        let g = result.unwrap();
        assert_eq!(g.key, "port:8080");
        assert_eq!(g.count, 1);
    }

    #[test]
    fn test_duplicate_submission_returns_none() {
        let mut agg = Aggregator::new(Duration::from_secs(60));
        agg.submit("port:8080", "first");
        let result = agg.submit("port:8080", "second");
        assert!(result.is_none());
    }

    #[test]
    fn test_count_increments_on_duplicates() {
        let mut agg = Aggregator::new(Duration::from_secs(60));
        agg.submit("port:9090", "a");
        agg.submit("port:9090", "b");
        agg.submit("port:9090", "c");
        let groups = agg.active_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].count, 3);
    }

    #[test]
    fn test_multiple_keys_tracked_independently() {
        let mut agg = Aggregator::new(Duration::from_secs(60));
        agg.submit("port:80", "http");
        agg.submit("port:443", "https");
        agg.submit("port:80", "http again");
        assert_eq!(agg.group_count(), 2);
    }

    #[test]
    fn test_flush_clears_state() {
        let mut agg = Aggregator::new(Duration::from_secs(60));
        agg.submit("port:22", "ssh");
        agg.submit("port:22", "ssh2");
        let flushed = agg.flush();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].count, 2);
        assert_eq!(agg.group_count(), 0);
    }

    #[test]
    fn test_expired_entries_evicted_on_submit() {
        let mut agg = Aggregator::new(Duration::from_millis(1));
        agg.submit("port:3000", "old");
        std::thread::sleep(Duration::from_millis(5));
        // New submission should evict the old one and register as new
        let result = agg.submit("port:3000", "new");
        assert!(result.is_some());
        assert_eq!(agg.group_count(), 1);
    }

    #[test]
    fn test_active_groups_empty_initially() {
        let agg = Aggregator::new(Duration::from_secs(10));
        assert!(agg.active_groups().is_empty());
    }
}

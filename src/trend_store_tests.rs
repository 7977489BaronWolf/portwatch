#[cfg(test)]
mod tests {
    use super::super::trend::TrendTracker;
    use super::super::trend_store::TrendStore;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_persist_and_reload() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("trend.json");
        let store = TrendStore::new(&path, 86400);

        let mut tracker = TrendTracker::new(20);
        tracker.record(5);
        tracker.record(10);
        tracker.record(15);
        store.persist(&tracker).unwrap();

        let mut tracker2 = TrendTracker::new(20);
        store.load_into(&mut tracker2);
        assert_eq!(tracker2.history().len(), 3);
    }

    #[test]
    fn test_load_missing_file_is_noop() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let store = TrendStore::new(&path, 86400);
        let mut tracker = TrendTracker::new(10);
        store.load_into(&mut tracker); // should not panic
        assert_eq!(tracker.history().len(), 0);
    }

    #[test]
    fn test_persist_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sub").join("dir").join("trend.json");
        let store = TrendStore::new(&path, 86400);
        let mut tracker = TrendTracker::new(10);
        tracker.record(3);
        assert!(store.persist(&tracker).is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_trend_analysis_after_reload() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("trend.json");
        let store = TrendStore::new(&path, 86400);

        let mut tracker = TrendTracker::new(20);
        tracker.record(2);
        tracker.record(8);
        store.persist(&tracker).unwrap();

        let mut tracker2 = TrendTracker::new(20);
        store.load_into(&mut tracker2);
        let summary = tracker2.analyze(Duration::from_secs(3600));
        assert!(summary.is_some());
    }

    #[test]
    fn test_corrupt_file_is_ignored() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("trend.json");
        std::fs::write(&path, b"not json at all").unwrap();
        let store = TrendStore::new(&path, 86400);
        let mut tracker = TrendTracker::new(10);
        store.load_into(&mut tracker);
        assert_eq!(tracker.history().len(), 0);
    }
}

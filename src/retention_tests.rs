#[cfg(test)]
mod tests {
    use super::super::retention::RetentionPolicy;
    use std::fs;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tempfile::tempdir;

    fn touch(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        fs::write(&path, b"data").unwrap();
        path
    }

    fn set_mtime_old(path: &std::path::Path, days_old: u64) {
        let old_time = SystemTime::now()
            .checked_sub(Duration::from_secs(days_old * 86_400))
            .unwrap();
        let ft = filetime::FileTime::from_system_time(old_time);
        filetime::set_file_mtime(path, ft).unwrap();
    }

    #[test]
    fn test_no_files_removed_when_under_limits() {
        let dir = tempdir().unwrap();
        touch(dir.path(), "snap_001.json");
        touch(dir.path(), "snap_002.json");

        let policy = RetentionPolicy::new(10, Some(90));
        let removed = policy.apply(dir.path(), "snap_").unwrap();
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_max_count_removes_oldest() {
        let dir = tempdir().unwrap();
        for i in 1..=5 {
            touch(dir.path(), &format!("snap_{:03}.json", i));
        }

        let policy = RetentionPolicy::new(3, None);
        let removed = policy.apply(dir.path(), "snap_").unwrap();
        assert_eq!(removed, 2);

        let remaining: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(remaining.len(), 3);
    }

    #[test]
    fn test_max_age_removes_stale_files() {
        let dir = tempdir().unwrap();
        let old = touch(dir.path(), "snap_old.json");
        set_mtime_old(&old, 100);
        touch(dir.path(), "snap_new.json");

        let policy = RetentionPolicy::new(0, Some(30));
        let removed = policy.apply(dir.path(), "snap_").unwrap();
        assert_eq!(removed, 1);
        assert!(!old.exists());
    }

    #[test]
    fn test_prefix_filter_ignores_unrelated_files() {
        let dir = tempdir().unwrap();
        touch(dir.path(), "snap_001.json");
        touch(dir.path(), "audit_001.json");

        let policy = RetentionPolicy::new(0, None);
        let removed = policy.apply(dir.path(), "snap_").unwrap();
        assert_eq!(removed, 0);
        assert!(dir.path().join("audit_001.json").exists());
    }

    #[test]
    fn test_default_policy_values() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.max_count, 30);
        assert!(policy.max_age.is_some());
    }
}

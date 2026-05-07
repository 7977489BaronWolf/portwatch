#[cfg(test)]
mod tests {
    use super::*;
    use crate::correlation_store::CorrelationStore;
    use crate::correlation::{Incident, CorrelationGroup};
    use crate::diff_engine::{PortDiff, DiffKind};
    use std::time::SystemTime;
    use tempfile::NamedTempFile;

    fn make_incident(id: &str, changes: usize) -> Incident {
        let diffs = (0..changes)
            .map(|i| PortDiff {
                port: 8000 + i as u16,
                kind: DiffKind::Opened,
                protocol: "tcp".to_string(),
                process: None,
            })
            .collect();
        Incident {
            id: id.to_string(),
            group: CorrelationGroup::NewService,
            diffs,
            detected_at: SystemTime::now(),
            summary: format!("{} new ports detected", changes),
        }
    }

    #[test]
    fn test_append_and_count() {
        let tmp = NamedTempFile::new().unwrap();
        let store = CorrelationStore::new(tmp.path());
        let inc = make_incident("inc-001", 3);
        store.append(&inc).unwrap();
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_multiple_appends() {
        let tmp = NamedTempFile::new().unwrap();
        let store = CorrelationStore::new(tmp.path());
        store.append(&make_incident("inc-001", 1)).unwrap();
        store.append(&make_incident("inc-002", 2)).unwrap();
        store.append(&make_incident("inc-003", 5)).unwrap();
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn test_load_ids() {
        let tmp = NamedTempFile::new().unwrap();
        let store = CorrelationStore::new(tmp.path());
        store.append(&make_incident("inc-aaa", 1)).unwrap();
        store.append(&make_incident("inc-bbb", 2)).unwrap();
        let ids = store.load_ids().unwrap();
        assert!(ids.contains(&"inc-aaa".to_string()));
        assert!(ids.contains(&"inc-bbb".to_string()));
    }

    #[test]
    fn test_empty_store_returns_zero() {
        let tmp = NamedTempFile::new().unwrap();
        let store = CorrelationStore::new(tmp.path());
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_nonexistent_file_load_ids() {
        let store = CorrelationStore::new("/tmp/portwatch_nonexistent_corr.jsonl");
        let ids = store.load_ids().unwrap();
        assert!(ids.is_empty());
    }
}

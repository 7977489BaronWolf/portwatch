#[cfg(test)]
mod tests {
    use super::super::snapshot::{Snapshot, SnapshotStore};
    use super::super::port_scanner::PortEntry;
    use tempfile::tempdir;

    fn make_entry(port: u16, proto: &str) -> PortEntry {
        PortEntry {
            port,
            protocol: proto.to_string(),
            process: Some("test".to_string()),
            pid: Some(1234),
        }
    }

    #[test]
    fn test_snapshot_new_has_ports() {
        let ports = vec![make_entry(80, "tcp"), make_entry(443, "tcp")];
        let snap = Snapshot::new(ports.clone(), Some("initial".to_string()));
        assert_eq!(snap.ports.len(), 2);
        assert_eq!(snap.label.as_deref(), Some("initial"));
        assert!(snap.timestamp > 0);
    }

    #[test]
    fn test_snapshot_port_map() {
        let ports = vec![make_entry(22, "tcp"), make_entry(8080, "tcp")];
        let snap = Snapshot::new(ports, None);
        let map = snap.port_map();
        assert!(map.contains_key(&22));
        assert!(map.contains_key(&8080));
        assert!(!map.contains_key(&80));
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let snap = Snapshot::new(vec![make_entry(80, "tcp")], Some("test".to_string()));
        store.save(&snap).unwrap();
        let loaded = store.load(snap.id).unwrap();
        assert_eq!(loaded.id, snap.id);
        assert_eq!(loaded.ports.len(), 1);
        assert_eq!(loaded.ports[0].port, 80);
    }

    #[test]
    fn test_list_sorted_by_timestamp() {
        let dir = tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let s1 = Snapshot { id: 100, timestamp: 100, label: None, ports: vec![] };
        let s2 = Snapshot { id: 200, timestamp: 200, label: None, ports: vec![] };
        store.save(&s2).unwrap();
        store.save(&s1).unwrap();
        let list = store.list().unwrap();
        assert_eq!(list[0].id, 100);
        assert_eq!(list[1].id, 200);
    }

    #[test]
    fn test_latest_returns_most_recent() {
        let dir = tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let s1 = Snapshot { id: 1, timestamp: 1, label: None, ports: vec![] };
        let s2 = Snapshot { id: 2, timestamp: 2, label: None, ports: vec![] };
        store.save(&s1).unwrap();
        store.save(&s2).unwrap();
        let latest = store.latest().unwrap().unwrap();
        assert_eq!(latest.id, 2);
    }

    #[test]
    fn test_delete_snapshot() {
        let dir = tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        let snap = Snapshot::new(vec![], None);
        store.save(&snap).unwrap();
        store.delete(snap.id).unwrap();
        assert!(store.load(snap.id).is_err());
    }

    #[test]
    fn test_latest_empty_store() {
        let dir = tempdir().unwrap();
        let store = SnapshotStore::new(dir.path());
        assert!(store.latest().unwrap().is_none());
    }
}

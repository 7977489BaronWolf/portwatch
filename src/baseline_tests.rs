#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::baseline::{Baseline, BaselineManager};
    use crate::port_scanner::PortInfo;
    use crate::state_store::StateStore;
    use tempfile::tempdir;

    fn make_port_info(port: u16) -> PortInfo {
        PortInfo { port, protocol: "tcp".to_string(), process: None }
    }

    #[test]
    fn test_baseline_from_port_infos() {
        let infos = vec![make_port_info(80), make_port_info(443), make_port_info(8080)];
        let baseline = Baseline::from_port_infos(&infos, "test");
        assert_eq!(baseline.ports.len(), 3);
        assert!(baseline.contains(80));
        assert!(baseline.contains(443));
        assert!(!baseline.contains(22));
    }

    #[test]
    fn test_unexpected_ports() {
        let baseline = Baseline::new(HashSet::from([80, 443]), "test");
        let current = HashSet::from([80, 443, 9000]);
        let unexpected = baseline.unexpected_ports(&current);
        assert_eq!(unexpected, HashSet::from([9000]));
    }

    #[test]
    fn test_missing_ports() {
        let baseline = Baseline::new(HashSet::from([80, 443, 22]), "test");
        let current = HashSet::from([80, 443]);
        let missing = baseline.missing_ports(&current);
        assert_eq!(missing, HashSet::from([22]));
    }

    #[test]
    fn test_no_changes() {
        let baseline = Baseline::new(HashSet::from([80, 443]), "test");
        let current = HashSet::from([80, 443]);
        assert!(baseline.unexpected_ports(&current).is_empty());
        assert!(baseline.missing_ports(&current).is_empty());
    }

    #[test]
    fn test_baseline_manager_save_load() {
        let dir = tempdir().unwrap();
        let store = StateStore::new(dir.path()).unwrap();
        let manager = BaselineManager::new(store);
        let baseline = Baseline::new(HashSet::from([80, 443, 22]), "initial");
        manager.save(&baseline).unwrap();
        let loaded = manager.load().unwrap().expect("baseline should exist");
        assert_eq!(loaded.ports, baseline.ports);
        assert_eq!(loaded.label, "initial");
    }

    #[test]
    fn test_baseline_manager_clear() {
        let dir = tempdir().unwrap();
        let store = StateStore::new(dir.path()).unwrap();
        let manager = BaselineManager::new(store);
        let baseline = Baseline::new(HashSet::from([80]), "test");
        manager.save(&baseline).unwrap();
        manager.clear().unwrap();
        let loaded = manager.load().unwrap();
        assert!(loaded.is_none());
    }
}

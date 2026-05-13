#[cfg(test)]
mod tests {
    use super::super::checkpoint::*;
    use std::collections::HashMap;

    fn make_state(ports: &[u16]) -> HashMap<u16, String> {
        ports.iter().map(|&p| (p, "LISTEN".to_string())).collect()
    }

    #[test]
    fn test_checkpoint_creation() {
        let state = make_state(&[80, 443, 8080]);
        let cp = Checkpoint::new("initial", state.clone(), 1);
        assert_eq!(cp.label, "initial");
        assert_eq!(cp.sequence, 1);
        assert_eq!(cp.port_count(), 3);
        assert!(cp.timestamp > 0);
    }

    #[test]
    fn test_checkpoint_age() {
        let cp = Checkpoint::new("test", make_state(&[22]), 0);
        let age = cp.age_secs();
        assert!(age < 5, "age should be nearly zero");
    }

    #[test]
    fn test_store_save_and_latest() {
        let mut store = CheckpointStore::new(5);
        assert!(store.latest().is_none());

        store.save(Checkpoint::new("first", make_state(&[80]), 1));
        store.save(Checkpoint::new("second", make_state(&[443]), 2));

        let latest = store.latest().unwrap();
        assert_eq!(latest.label, "second");
    }

    #[test]
    fn test_store_max_retained() {
        let mut store = CheckpointStore::new(3);
        for i in 0..5u64 {
            store.save(Checkpoint::new(format!("cp{}", i), make_state(&[]), i));
        }
        assert_eq!(store.all().len(), 3);
        assert_eq!(store.latest().unwrap().label, "cp4");
    }

    #[test]
    fn test_find_by_label() {
        let mut store = CheckpointStore::new(10);
        store.save(Checkpoint::new("alpha", make_state(&[22]), 1));
        store.save(Checkpoint::new("beta", make_state(&[80]), 2));
        store.save(Checkpoint::new("alpha", make_state(&[443]), 3));

        let found = store.find_by_label("alpha").unwrap();
        assert_eq!(found.sequence, 3, "should find most recent alpha");

        assert!(store.find_by_label("gamma").is_none());
    }

    #[test]
    fn test_store_clear() {
        let mut store = CheckpointStore::new(5);
        store.save(Checkpoint::new("x", make_state(&[80]), 1));
        store.clear();
        assert!(store.latest().is_none());
        assert_eq!(store.all().len(), 0);
    }

    #[test]
    fn test_port_count_empty() {
        let cp = Checkpoint::new("empty", HashMap::new(), 0);
        assert_eq!(cp.port_count(), 0);
    }
}

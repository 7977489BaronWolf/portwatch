#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use tempfile::NamedTempFile;

    use crate::state_store::{PortDiff, PortState, StateStore};

    fn make_state(ports: &[u16], ts: u64) -> PortState {
        PortState::new(ports.iter().copied().collect(), ts)
    }

    #[test]
    fn test_diff_detects_opened_ports() {
        let old = make_state(&[80, 443], 100);
        let new = make_state(&[80, 443, 8080], 200);
        let diff = old.diff(&new);
        assert!(diff.opened.contains(&8080));
        assert!(diff.closed.is_empty());
    }

    #[test]
    fn test_diff_detects_closed_ports() {
        let old = make_state(&[80, 443, 8080], 100);
        let new = make_state(&[80, 443], 200);
        let diff = old.diff(&new);
        assert!(diff.closed.contains(&8080));
        assert!(diff.opened.is_empty());
    }

    #[test]
    fn test_diff_no_changes() {
        let old = make_state(&[80, 443], 100);
        let new = make_state(&[80, 443], 200);
        let diff = old.diff(&new);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_diff_both_opened_and_closed() {
        let old = make_state(&[80, 22], 100);
        let new = make_state(&[80, 443], 200);
        let diff = old.diff(&new);
        assert!(diff.opened.contains(&443));
        assert!(diff.closed.contains(&22));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let store = StateStore::new(tmp.path());
        let state = make_state(&[22, 80, 443], 999);

        store.save(&state).unwrap();
        let loaded = store.load().unwrap().expect("state should exist");

        assert_eq!(loaded.ports, state.ports);
        assert_eq!(loaded.timestamp, state.timestamp);
    }

    #[test]
    fn test_load_returns_none_when_file_missing() {
        let store = StateStore::new("/tmp/portwatch_nonexistent_state_xyz.json");
        let result = store.load().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_port_diff_is_empty() {
        let diff = PortDiff {
            opened: HashSet::new(),
            closed: HashSet::new(),
        };
        assert!(diff.is_empty());
    }
}

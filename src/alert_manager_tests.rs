#[cfg(test)]
mod tests {
    use crate::alert_manager::AlertManager;
    use crate::config::Config;
    use crate::diff_engine::{PortDiff, PortEntry};

    fn make_config(cooldown: u64) -> Config {
        Config {
            scan_interval_secs: 5,
            alert_cooldown_secs: Some(cooldown),
            notification_hooks: vec![],
            allowed_ports: vec![],
            state_file: None,
        }
    }

    fn make_opened_diff(port: u16) -> PortDiff {
        PortDiff::Opened(PortEntry {
            port,
            protocol: "tcp".to_string(),
            pid: Some(1234),
            process: Some("nginx".to_string()),
        })
    }

    fn make_closed_diff(port: u16) -> PortDiff {
        PortDiff::Closed(PortEntry {
            port,
            protocol: "tcp".to_string(),
            pid: None,
            process: None,
        })
    }

    #[test]
    fn test_process_diffs_no_panic() {
        let config = make_config(60);
        let mut mgr = AlertManager::new(&config);
        let diffs = vec![make_opened_diff(8080), make_closed_diff(22)];
        // Should not panic even with no real notifier hooks configured
        mgr.process_diffs(&diffs);
    }

    #[test]
    fn test_throttle_prevents_double_alert() {
        let config = make_config(3600); // 1 hour cooldown
        let mut mgr = AlertManager::new(&config);
        let diffs = vec![make_opened_diff(9090)];

        mgr.process_diffs(&diffs);
        let count_before = mgr.last_alert.len();
        mgr.process_diffs(&diffs); // should be throttled
        let count_after = mgr.last_alert.len();

        assert_eq!(count_before, count_after);
    }

    #[test]
    fn test_reset_throttle_clears_state() {
        let config = make_config(3600);
        let mut mgr = AlertManager::new(&config);
        mgr.process_diffs(&[make_opened_diff(443)]);
        assert!(!mgr.last_alert.is_empty());
        mgr.reset_throttle();
        assert!(mgr.last_alert.is_empty());
    }

    #[test]
    fn test_zero_cooldown_always_alerts() {
        let config = make_config(0);
        let mut mgr = AlertManager::new(&config);
        let diffs = vec![make_opened_diff(3000)];
        mgr.process_diffs(&diffs);
        mgr.process_diffs(&diffs); // should alert again immediately
    }
}

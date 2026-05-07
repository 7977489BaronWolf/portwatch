#[cfg(test)]
mod tests {
    use super::super::silencer::Silencer;
    use std::time::Duration;

    #[test]
    fn test_silence_and_is_silenced() {
        let mut s = Silencer::new();
        s.silence(8080, Duration::from_secs(60), "maintenance window");
        assert!(s.is_silenced(8080));
    }

    #[test]
    fn test_unsilenced_port_not_silenced() {
        let mut s = Silencer::new();
        assert!(!s.is_silenced(443));
    }

    #[test]
    fn test_unsilence_removes_rule() {
        let mut s = Silencer::new();
        s.silence(22, Duration::from_secs(60), "ssh maintenance");
        assert!(s.is_silenced(22));
        let removed = s.unsilence(22);
        assert!(removed);
        assert!(!s.is_silenced(22));
    }

    #[test]
    fn test_unsilence_nonexistent_returns_false() {
        let mut s = Silencer::new();
        assert!(!s.unsilence(9999));
    }

    #[test]
    fn test_expired_rule_is_not_silenced() {
        let mut s = Silencer::new();
        // Zero duration expires immediately
        s.silence(3000, Duration::from_nanos(1), "instant expiry");
        std::thread::sleep(Duration::from_millis(5));
        assert!(!s.is_silenced(3000));
    }

    #[test]
    fn test_purge_expired_clears_stale_rules() {
        let mut s = Silencer::new();
        s.silence(1000, Duration::from_nanos(1), "stale");
        s.silence(2000, Duration::from_secs(60), "active");
        std::thread::sleep(Duration::from_millis(5));
        s.purge_expired();
        assert_eq!(s.active_count(), 1);
    }

    #[test]
    fn test_active_rules_returns_only_live_rules() {
        let mut s = Silencer::new();
        s.silence(80, Duration::from_secs(60), "web maintenance");
        s.silence(443, Duration::from_secs(60), "tls maintenance");
        let rules = s.active_rules();
        assert_eq!(rules.len(), 2);
        let ports: Vec<u16> = rules.iter().map(|r| r.port).collect();
        assert!(ports.contains(&80));
        assert!(ports.contains(&443));
    }

    #[test]
    fn test_silence_reason_preserved() {
        let mut s = Silencer::new();
        s.silence(5432, Duration::from_secs(30), "db migration");
        let rules = s.active_rules();
        assert_eq!(rules[0].reason, "db migration");
    }

    #[test]
    fn test_multiple_ports_independently_silenced() {
        let mut s = Silencer::new();
        s.silence(8080, Duration::from_secs(60), "app");
        s.silence(9090, Duration::from_secs(60), "metrics");
        assert!(s.is_silenced(8080));
        assert!(s.is_silenced(9090));
        assert!(!s.is_silenced(7070));
    }
}

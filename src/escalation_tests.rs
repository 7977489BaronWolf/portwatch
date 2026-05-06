#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::escalation::{EscalationConfig, EscalationTracker};

    fn default_tracker() -> EscalationTracker {
        EscalationTracker::new(EscalationConfig {
            threshold: 3,
            window: Duration::from_secs(60),
        })
    }

    #[test]
    fn test_no_escalation_below_threshold() {
        let mut tracker = default_tracker();
        assert!(!tracker.record("port:8080:opened"));
        assert!(!tracker.record("port:8080:opened"));
    }

    #[test]
    fn test_escalation_at_threshold() {
        let mut tracker = default_tracker();
        tracker.record("port:443:closed");
        tracker.record("port:443:closed");
        let escalated = tracker.record("port:443:closed");
        assert!(escalated, "should escalate at threshold");
    }

    #[test]
    fn test_escalation_fires_only_once() {
        let mut tracker = default_tracker();
        for _ in 0..3 {
            tracker.record("port:22:opened");
        }
        // Fourth call should NOT escalate again
        let second_escalation = tracker.record("port:22:opened");
        assert!(!second_escalation, "escalation should only fire once per window");
    }

    #[test]
    fn test_count_increments() {
        let mut tracker = default_tracker();
        tracker.record("port:80:opened");
        tracker.record("port:80:opened");
        assert_eq!(tracker.count("port:80:opened"), 2);
    }

    #[test]
    fn test_acknowledge_resets_state() {
        let mut tracker = default_tracker();
        tracker.record("port:3306:opened");
        tracker.record("port:3306:opened");
        tracker.acknowledge("port:3306:opened");
        assert_eq!(tracker.count("port:3306:opened"), 0);
    }

    #[test]
    fn test_independent_keys() {
        let mut tracker = default_tracker();
        tracker.record("port:8080:opened");
        tracker.record("port:8080:opened");
        tracker.record("port:8080:opened");
        // Different key should not be affected
        assert!(!tracker.record("port:9090:opened"));
        assert_eq!(tracker.count("port:9090:opened"), 1);
    }

    #[test]
    fn test_unknown_key_count_is_zero() {
        let tracker = default_tracker();
        assert_eq!(tracker.count("port:9999:closed"), 0);
    }
}

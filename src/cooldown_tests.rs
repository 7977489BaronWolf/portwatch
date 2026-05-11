#[cfg(test)]
mod tests {
    use super::super::cooldown::CooldownTracker;
    use std::time::Duration;

    #[test]
    fn first_call_is_allowed() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(60));
        assert!(tracker.allow("port:8080:opened"));
    }

    #[test]
    fn second_immediate_call_is_blocked() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(60));
        assert!(tracker.allow("port:8080:opened"));
        assert!(!tracker.allow("port:8080:opened"));
    }

    #[test]
    fn different_keys_are_independent() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(60));
        assert!(tracker.allow("port:8080:opened"));
        assert!(tracker.allow("port:9090:opened"));
        assert!(!tracker.allow("port:8080:opened"));
        assert!(!tracker.allow("port:9090:opened"));
    }

    #[test]
    fn reset_allows_immediate_refire() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(60));
        assert!(tracker.allow("port:443:closed"));
        assert!(!tracker.allow("port:443:closed"));
        tracker.reset("port:443:closed");
        assert!(tracker.allow("port:443:closed"));
    }

    #[test]
    fn zero_window_always_allows() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(0));
        assert!(tracker.allow("port:22:opened"));
        // With a zero window the elapsed time is always >= window.
        assert!(tracker.allow("port:22:opened"));
    }

    #[test]
    fn len_tracks_unique_keys() {
        let mut tracker = CooldownTracker::new(Duration::from_secs(60));
        assert_eq!(tracker.len(), 0);
        tracker.allow("a");
        tracker.allow("b");
        tracker.allow("a"); // blocked, but key already present
        assert_eq!(tracker.len(), 2);
    }

    #[test]
    fn evict_expired_removes_old_entries() {
        let mut tracker = CooldownTracker::new(Duration::from_millis(1));
        tracker.allow("port:80:opened");
        assert_eq!(tracker.len(), 1);
        std::thread::sleep(Duration::from_millis(5));
        tracker.evict_expired();
        assert!(tracker.is_empty());
    }

    #[test]
    fn is_empty_on_fresh_tracker() {
        let tracker = CooldownTracker::new(Duration::from_secs(30));
        assert!(tracker.is_empty());
    }
}

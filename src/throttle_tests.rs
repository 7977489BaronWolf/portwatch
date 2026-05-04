#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use crate::throttle::{Throttle, ThrottleKey};

    fn key(port: u16, kind: &str) -> ThrottleKey {
        ThrottleKey::new(port, kind)
    }

    #[test]
    fn first_alert_always_allowed() {
        let mut t = Throttle::new(Duration::from_secs(60));
        assert!(t.allow(&key(8080, "opened")));
    }

    #[test]
    fn second_alert_blocked_within_cooldown() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let k = key(8080, "opened");
        assert!(t.allow(&k));
        assert!(!t.allow(&k), "second call should be throttled");
    }

    #[test]
    fn different_keys_are_independent() {
        let mut t = Throttle::new(Duration::from_secs(60));
        assert!(t.allow(&key(8080, "opened")));
        assert!(t.allow(&key(9090, "opened")));
        assert!(t.allow(&key(8080, "closed")));
    }

    #[test]
    fn alert_allowed_after_cooldown_expires() {
        let mut t = Throttle::new(Duration::from_millis(50));
        let k = key(443, "closed");
        assert!(t.allow(&k));
        assert!(!t.allow(&k));
        thread::sleep(Duration::from_millis(60));
        assert!(t.allow(&k), "should be allowed after cooldown");
    }

    #[test]
    fn reset_clears_specific_key() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let k = key(22, "opened");
        assert!(t.allow(&k));
        assert!(!t.allow(&k));
        t.reset(&k);
        assert!(t.allow(&k), "should be allowed after explicit reset");
    }

    #[test]
    fn reset_all_clears_everything() {
        let mut t = Throttle::new(Duration::from_secs(60));
        t.allow(&key(80, "opened"));
        t.allow(&key(443, "opened"));
        t.reset_all();
        assert!(t.allow(&key(80, "opened")));
        assert!(t.allow(&key(443, "opened")));
    }

    #[test]
    fn remaining_returns_none_when_not_fired() {
        let t = Throttle::new(Duration::from_secs(60));
        assert!(t.remaining(&key(80, "opened")).is_none());
    }

    #[test]
    fn remaining_returns_some_within_cooldown() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let k = key(80, "opened");
        t.allow(&k);
        let r = t.remaining(&k);
        assert!(r.is_some());
        assert!(r.unwrap() <= Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::super::rate_limit::RateLimiter;
    use std::time::Duration;

    fn limiter(max: usize, secs: u64) -> RateLimiter {
        RateLimiter::new(max, Duration::from_secs(secs))
    }

    #[test]
    fn allows_up_to_max_notifications() {
        let mut rl = limiter(3, 60);
        assert!(rl.allow("webhook"));
        assert!(rl.allow("webhook"));
        assert!(rl.allow("webhook"));
    }

    #[test]
    fn blocks_after_max_exceeded() {
        let mut rl = limiter(2, 60);
        assert!(rl.allow("email"));
        assert!(rl.allow("email"));
        assert!(!rl.allow("email"), "should be blocked on 3rd call");
    }

    #[test]
    fn channels_are_independent() {
        let mut rl = limiter(1, 60);
        assert!(rl.allow("slack"));
        assert!(!rl.allow("slack"));
        // A different channel should not be affected.
        assert!(rl.allow("pagerduty"));
    }

    #[test]
    fn remaining_decrements_correctly() {
        let mut rl = limiter(5, 60);
        assert_eq!(rl.remaining("webhook"), 5);
        rl.allow("webhook");
        assert_eq!(rl.remaining("webhook"), 4);
        rl.allow("webhook");
        rl.allow("webhook");
        assert_eq!(rl.remaining("webhook"), 2);
    }

    #[test]
    fn remaining_is_zero_when_exhausted() {
        let mut rl = limiter(2, 60);
        rl.allow("ch");
        rl.allow("ch");
        assert_eq!(rl.remaining("ch"), 0);
    }

    #[test]
    fn reset_clears_history_for_channel() {
        let mut rl = limiter(1, 60);
        rl.allow("hook");
        assert!(!rl.allow("hook"));
        rl.reset("hook");
        assert!(rl.allow("hook"), "should be allowed after reset");
    }

    #[test]
    fn very_short_window_expires_quickly() {
        let mut rl = RateLimiter::new(1, Duration::from_millis(50));
        assert!(rl.allow("fast"));
        assert!(!rl.allow("fast"));
        std::thread::sleep(Duration::from_millis(60));
        assert!(rl.allow("fast"), "window should have expired");
    }
}

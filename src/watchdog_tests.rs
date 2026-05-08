#[cfg(test)]
mod tests {
    use super::super::watchdog::{Watchdog};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn healthy_immediately_after_creation() {
        let wd = Watchdog::new(30);
        assert!(wd.check().is_ok(), "should be healthy right after creation");
    }

    #[test]
    fn elapsed_within_timeout_is_ok() {
        let wd = Watchdog::new(60);
        let result = wd.check();
        assert!(result.is_ok());
        // elapsed should be 0 or 1 second at most
        assert!(result.unwrap() <= 2);
    }

    #[test]
    fn handle_beat_resets_timestamp() {
        let wd = Watchdog::new(5);
        let handle = wd.handle();
        // Simulate time passing by checking before and after a beat
        handle.beat();
        let result = wd.check();
        assert!(result.is_ok(), "should be healthy after a beat");
    }

    #[test]
    fn stalled_when_timeout_exceeded() {
        // Use a 1-second timeout and sleep 2 seconds
        let wd = Watchdog::new(1);
        thread::sleep(Duration::from_secs(2));
        let result = wd.check();
        assert!(result.is_err(), "should be stalled after timeout exceeded");
        let elapsed = result.unwrap_err();
        assert!(elapsed >= 2, "elapsed should be >= 2, got {}", elapsed);
    }

    #[test]
    fn reset_clears_stall() {
        let wd = Watchdog::new(1);
        thread::sleep(Duration::from_secs(2));
        assert!(wd.check().is_err(), "should be stalled before reset");
        wd.reset();
        assert!(wd.check().is_ok(), "should be healthy after reset");
    }

    #[test]
    fn multiple_handles_share_state() {
        let wd = Watchdog::new(1);
        let h1 = wd.handle();
        let h2 = wd.handle();
        thread::sleep(Duration::from_secs(2));
        assert!(wd.check().is_err());
        // Beat via h2 — should affect the shared watchdog
        h2.beat();
        assert!(wd.check().is_ok(), "h2 beat should have updated shared state");
        // h1 also works
        thread::sleep(Duration::from_secs(2));
        h1.beat();
        assert!(wd.check().is_ok(), "h1 beat should also update shared state");
    }

    #[test]
    fn timeout_secs_accessor() {
        let wd = Watchdog::new(42);
        assert_eq!(wd.timeout_secs(), 42);
    }
}

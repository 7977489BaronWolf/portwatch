#[cfg(test)]
mod tests {
    use super::super::scheduler::Scheduler;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_is_due_on_first_call() {
        let s = Scheduler::with_interval(Duration::from_secs(60));
        assert!(s.is_due(), "should be due when never run");
    }

    #[test]
    fn test_not_due_immediately_after_mark() {
        let mut s = Scheduler::with_interval(Duration::from_secs(60));
        s.mark_run();
        assert!(!s.is_due(), "should not be due right after marking run");
    }

    #[test]
    fn test_due_after_interval_elapsed() {
        let mut s = Scheduler::with_interval(Duration::from_millis(50));
        s.mark_run();
        thread::sleep(Duration::from_millis(60));
        assert!(s.is_due(), "should be due after interval has elapsed");
    }

    #[test]
    fn test_mark_run_updates_last_run() {
        let mut s = Scheduler::with_interval(Duration::from_millis(50));
        s.mark_run();
        thread::sleep(Duration::from_millis(60));
        assert!(s.is_due());
        s.mark_run();
        assert!(!s.is_due(), "should not be due immediately after re-marking");
    }

    #[test]
    fn test_interval_accessor() {
        let s = Scheduler::with_interval(Duration::from_secs(30));
        assert_eq!(s.interval(), Duration::from_secs(30));
    }

    #[test]
    fn test_wait_for_next_tick_does_not_block_on_first_call() {
        let mut s = Scheduler::with_interval(Duration::from_secs(60));
        let before = std::time::Instant::now();
        s.wait_for_next_tick();
        // First tick should return almost immediately (no prior run)
        assert!(before.elapsed() < Duration::from_millis(100));
    }
}

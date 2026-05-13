#[cfg(test)]
mod tests {
    use super::super::debounce::Debouncer;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn first_call_always_emits() {
        let mut d = Debouncer::new(Duration::from_secs(5));
        assert!(d.should_emit("port:8080:opened"));
    }

    #[test]
    fn second_call_within_window_suppressed() {
        let mut d = Debouncer::new(Duration::from_secs(5));
        assert!(d.should_emit("port:443:closed"));
        assert!(!d.should_emit("port:443:closed"));
    }

    #[test]
    fn different_keys_are_independent() {
        let mut d = Debouncer::new(Duration::from_secs(5));
        assert!(d.should_emit("port:80:opened"));
        assert!(d.should_emit("port:22:opened"));
        assert!(!d.should_emit("port:80:opened"));
        assert!(!d.should_emit("port:22:opened"));
    }

    #[test]
    fn emits_after_window_expires() {
        let mut d = Debouncer::new(Duration::from_millis(50));
        assert!(d.should_emit("port:9000:opened"));
        thread::sleep(Duration::from_millis(60));
        assert!(d.should_emit("port:9000:opened"));
    }

    #[test]
    fn reset_allows_immediate_re_emit() {
        let mut d = Debouncer::new(Duration::from_secs(60));
        assert!(d.should_emit("port:3000:opened"));
        assert!(!d.should_emit("port:3000:opened"));
        d.reset("port:3000:opened");
        assert!(d.should_emit("port:3000:opened"));
    }

    #[test]
    fn clear_resets_all_keys() {
        let mut d = Debouncer::new(Duration::from_secs(60));
        d.should_emit("a");
        d.should_emit("b");
        assert_eq!(d.tracked_count(), 2);
        d.clear();
        assert_eq!(d.tracked_count(), 0);
        assert!(d.should_emit("a"));
    }

    #[test]
    fn tracked_count_reflects_unique_keys() {
        let mut d = Debouncer::new(Duration::from_secs(5));
        d.should_emit("x");
        d.should_emit("x");
        d.should_emit("y");
        assert_eq!(d.tracked_count(), 2);
    }
}

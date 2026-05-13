#[cfg(test)]
mod tests {
    use super::super::backoff::{Backoff, BackoffConfig};
    use std::time::Duration;

    fn config() -> BackoffConfig {
        BackoffConfig {
            initial_delay_ms: 100,
            max_delay_ms: 1_000,
            multiplier: 2.0,
            max_attempts: 4,
        }
    }

    #[test]
    fn first_delay_is_initial() {
        let mut b = Backoff::new(config());
        assert_eq!(b.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(b.attempt(), 1);
    }

    #[test]
    fn delays_grow_exponentially() {
        let mut b = Backoff::new(config());
        let d0 = b.next_delay().unwrap().as_millis();
        let d1 = b.next_delay().unwrap().as_millis();
        let d2 = b.next_delay().unwrap().as_millis();
        assert_eq!(d0, 100);
        assert_eq!(d1, 200);
        assert_eq!(d2, 400);
    }

    #[test]
    fn delay_capped_at_max() {
        let cfg = BackoffConfig {
            initial_delay_ms: 500,
            max_delay_ms: 600,
            multiplier: 3.0,
            max_attempts: 5,
        };
        let mut b = Backoff::new(cfg);
        b.next_delay(); // 500
        let capped = b.next_delay().unwrap().as_millis();
        assert!(capped <= 600);
    }

    #[test]
    fn returns_none_after_max_attempts() {
        let mut b = Backoff::new(config());
        for _ in 0..4 {
            assert!(b.next_delay().is_some());
        }
        assert!(b.next_delay().is_none());
        assert!(b.exhausted());
    }

    #[test]
    fn reset_restarts_sequence() {
        let mut b = Backoff::new(config());
        for _ in 0..4 {
            b.next_delay();
        }
        b.reset();
        assert_eq!(b.attempt(), 0);
        assert!(!b.exhausted());
        assert_eq!(b.next_delay(), Some(Duration::from_millis(100)));
    }

    #[test]
    fn defaults_are_reasonable() {
        let mut b = Backoff::with_defaults();
        let d = b.next_delay().unwrap();
        assert_eq!(d.as_millis(), 250);
    }
}

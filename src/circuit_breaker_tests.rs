#[cfg(test)]
mod tests {
    use super::super::circuit_breaker::*;
    use std::time::Duration;

    fn make_breaker() -> CircuitBreaker {
        CircuitBreaker::new(3, 2, Duration::from_secs(60))
    }

    #[test]
    fn test_initial_state_is_closed() {
        let cb = make_breaker();
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_allows_request_when_closed() {
        let mut cb = make_breaker();
        assert!(cb.allow_request());
    }

    #[test]
    fn test_opens_after_threshold_failures() {
        let mut cb = make_breaker();
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Closed);
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    #[test]
    fn test_blocks_request_when_open() {
        let mut cb = make_breaker();
        cb.record_failure();
        cb.record_failure();
        cb.record_failure();
        assert!(!cb.allow_request());
        assert!(cb.is_open());
    }

    #[test]
    fn test_success_resets_failure_count() {
        let mut cb = make_breaker();
        cb.record_failure();
        cb.record_failure();
        cb.record_success();
        assert_eq!(cb.state(), &CircuitState::Closed);
        // After reset, need threshold failures again to open
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Closed);
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    #[test]
    fn test_half_open_closes_after_successes() {
        let mut cb = CircuitBreaker::new(1, 2, Duration::from_nanos(1));
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Open);
        // Wait for timeout
        std::thread::sleep(Duration::from_millis(5));
        assert!(cb.allow_request()); // transitions to HalfOpen
        assert_eq!(cb.state(), &CircuitState::HalfOpen);
        cb.record_success();
        assert_eq!(cb.state(), &CircuitState::HalfOpen);
        cb.record_success();
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_half_open_reopens_on_failure() {
        let mut cb = CircuitBreaker::new(1, 2, Duration::from_nanos(1));
        cb.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        cb.allow_request();
        assert_eq!(cb.state(), &CircuitState::HalfOpen);
        cb.record_failure();
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    #[test]
    fn test_registry_creates_breakers_per_channel() {
        let mut registry = CircuitBreakerRegistry::new(3, 2, Duration::from_secs(60));
        assert!(registry.allow_request("slack"));
        assert!(registry.allow_request("email"));
        registry.record_failure("slack");
        registry.record_failure("slack");
        registry.record_failure("slack");
        assert!(!registry.allow_request("slack"));
        assert!(registry.allow_request("email"));
    }

    #[test]
    fn test_registry_status() {
        let mut registry = CircuitBreakerRegistry::new(1, 1, Duration::from_secs(60));
        registry.record_failure("pagerduty");
        let status = registry.status();
        assert_eq!(status.get("pagerduty").map(|s| s.as_str()), Some("Open"));
    }
}

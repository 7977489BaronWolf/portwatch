#[cfg(test)]
mod tests {
    use super::super::quota::*;
    use std::time::Duration;

    fn make_manager(max: usize) -> QuotaManager {
        QuotaManager::new(QuotaConfig {
            max_per_window: max,
            window: Duration::from_secs(60),
        })
    }

    #[test]
    fn test_within_quota() {
        let mut qm = make_manager(3);
        assert!(qm.check_and_consume("slack"));
        assert!(qm.check_and_consume("slack"));
        assert!(qm.check_and_consume("slack"));
    }

    #[test]
    fn test_exceeds_quota() {
        let mut qm = make_manager(2);
        assert!(qm.check_and_consume("email"));
        assert!(qm.check_and_consume("email"));
        assert!(!qm.check_and_consume("email"));
    }

    #[test]
    fn test_remaining_decrements() {
        let mut qm = make_manager(5);
        assert_eq!(qm.remaining("pager"), 5);
        qm.check_and_consume("pager");
        qm.check_and_consume("pager");
        assert_eq!(qm.remaining("pager"), 3);
    }

    #[test]
    fn test_reset_clears_bucket() {
        let mut qm = make_manager(1);
        qm.check_and_consume("webhook");
        assert!(!qm.check_and_consume("webhook"));
        qm.reset("webhook");
        assert!(qm.check_and_consume("webhook"));
    }

    #[test]
    fn test_per_channel_config() {
        let mut qm = make_manager(10);
        qm.set_channel_config("sms", QuotaConfig {
            max_per_window: 2,
            window: Duration::from_secs(60),
        });
        assert!(qm.check_and_consume("sms"));
        assert!(qm.check_and_consume("sms"));
        assert!(!qm.check_and_consume("sms"));
        // global channel still has 10
        for _ in 0..10 {
            assert!(qm.check_and_consume("slack"));
        }
        assert!(!qm.check_and_consume("slack"));
    }

    #[test]
    fn test_independent_channels() {
        let mut qm = make_manager(1);
        assert!(qm.check_and_consume("a"));
        assert!(!qm.check_and_consume("a"));
        assert!(qm.check_and_consume("b"));
    }
}

#[cfg(test)]
mod tests {
    use super::super::quota::{QuotaConfig, QuotaManager};
    use super::super::quota_cmd::{QuotaCmd, QuotaCmdHandler};
    use std::time::Duration;

    fn handler() -> QuotaCmdHandler {
        let mgr = QuotaManager::new(QuotaConfig {
            max_per_window: 10,
            window: Duration::from_secs(3600),
        });
        QuotaCmdHandler::new(mgr)
    }

    #[test]
    fn test_status_no_channel() {
        let h = handler();
        let out = h.manager.remaining("x");
        assert_eq!(out, 10);
    }

    #[test]
    fn test_status_with_channel() {
        let mut h = handler();
        let out = h.handle(QuotaCmd::Status { channel: Some("slack".into()) });
        assert!(out.contains("slack"));
        assert!(out.contains("remaining=10"));
    }

    #[test]
    fn test_reset_cmd() {
        let mut h = handler();
        // consume some quota
        h.handle(QuotaCmd::SetLimit {
            channel: "email".into(),
            max: 3,
            window_secs: 60,
        });
        let out = h.handle(QuotaCmd::Reset { channel: "email".into() });
        assert!(out.contains("reset"));
        assert!(out.contains("email"));
    }

    #[test]
    fn test_set_limit_cmd() {
        let mut h = handler();
        let out = h.handle(QuotaCmd::SetLimit {
            channel: "sms".into(),
            max: 5,
            window_secs: 300,
        });
        assert!(out.contains("sms"));
        assert!(out.contains("max=5"));
        assert!(out.contains("window=300s"));
    }

    #[test]
    fn test_status_after_set_limit() {
        let mut h = handler();
        h.handle(QuotaCmd::SetLimit {
            channel: "pager".into(),
            max: 2,
            window_secs: 60,
        });
        let out = h.handle(QuotaCmd::Status { channel: Some("pager".into()) });
        assert!(out.contains("remaining=2"));
    }
}

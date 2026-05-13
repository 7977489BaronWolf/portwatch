#[cfg(test)]
mod tests {
    use super::super::sampling_cmd::*;
    use super::super::sampling::SamplingConfig;
    use std::time::Duration;

    fn handler() -> SamplingCmdHandler {
        SamplingCmdHandler::new(SamplingConfig {
            base_rate: 1.0,
            high_freq_threshold: 5,
            reduced_rate: 0.2,
            window: Duration::from_secs(60),
        })
    }

    #[test]
    fn test_status_unknown_key() {
        let mut h = handler();
        let out = h.handle(SamplingCmd::Status { key: Some("port:99".to_string()) });
        assert!(out.contains("No stats"));
    }

    #[test]
    fn test_status_after_sampling() {
        let mut h = handler();
        h.sampler.should_sample("port:80");
        h.sampler.should_sample("port:80");
        let out = h.handle(SamplingCmd::Status { key: Some("port:80".to_string()) });
        assert!(out.contains("count=2"));
        assert!(out.contains("sampled=2"));
    }

    #[test]
    fn test_status_no_key() {
        let mut h = handler();
        let out = h.handle(SamplingCmd::Status { key: None });
        assert!(out.contains("sampling status"));
    }

    #[test]
    fn test_reset_cmd() {
        let mut h = handler();
        h.sampler.should_sample("port:22");
        let out = h.handle(SamplingCmd::Reset { key: "port:22".to_string() });
        assert!(out.contains("Reset"));
        assert!(h.sampler.stats("port:22").is_none());
    }

    #[test]
    fn test_configure_cmd() {
        let mut h = handler();
        let out = h.handle(SamplingCmd::Configure {
            base_rate: 0.5,
            high_freq_threshold: 20,
            reduced_rate: 0.05,
            window_secs: 120,
        });
        assert!(out.contains("base_rate=0.5"));
        assert!(out.contains("threshold=20"));
        assert!(out.contains("window=120s"));
    }

    #[test]
    fn test_drop_rate_displayed_correctly() {
        let mut h = handler();
        // Trigger 10 events — first 5 kept, rest dropped (reduced_rate=0.2)
        for _ in 0..10 {
            h.sampler.should_sample("port:443");
        }
        let out = h.handle(SamplingCmd::Status { key: Some("port:443".to_string()) });
        assert!(out.contains("count=10"));
        assert!(out.contains("drop_rate="));
    }
}

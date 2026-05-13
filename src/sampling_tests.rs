#[cfg(test)]
mod tests {
    use super::super::sampling::*;
    use std::time::Duration;

    fn default_sampler() -> Sampler {
        Sampler::new(SamplingConfig::default())
    }

    #[test]
    fn test_base_rate_one_keeps_all() {
        let mut s = Sampler::new(SamplingConfig {
            base_rate: 1.0,
            high_freq_threshold: 100,
            reduced_rate: 0.5,
            window: Duration::from_secs(60),
        });
        for _ in 0..10 {
            assert!(s.should_sample("port:80"));
        }
        let (count, sampled) = s.stats("port:80").unwrap();
        assert_eq!(count, 10);
        assert_eq!(sampled, 10);
    }

    #[test]
    fn test_base_rate_zero_drops_all() {
        let mut s = Sampler::new(SamplingConfig {
            base_rate: 0.0,
            high_freq_threshold: 100,
            reduced_rate: 0.0,
            window: Duration::from_secs(60),
        });
        for _ in 0..10 {
            assert!(!s.should_sample("port:443"));
        }
        let (count, sampled) = s.stats("port:443").unwrap();
        assert_eq!(count, 10);
        assert_eq!(sampled, 0);
    }

    #[test]
    fn test_high_frequency_triggers_reduced_rate() {
        let mut s = Sampler::new(SamplingConfig {
            base_rate: 1.0,
            high_freq_threshold: 3,
            reduced_rate: 0.0,
            window: Duration::from_secs(60),
        });
        // First 3 events: base_rate=1.0 → all kept
        for _ in 0..3 {
            s.should_sample("port:22");
        }
        let (_, sampled_before) = s.stats("port:22").unwrap();
        assert_eq!(sampled_before, 3);

        // Events 4+ exceed threshold → reduced_rate=0.0 → none kept
        for _ in 0..5 {
            assert!(!s.should_sample("port:22"));
        }
    }

    #[test]
    fn test_different_keys_are_independent() {
        let mut s = default_sampler();
        s.should_sample("port:80");
        s.should_sample("port:443");
        assert!(s.stats("port:80").is_some());
        assert!(s.stats("port:443").is_some());
        assert!(s.stats("port:22").is_none());
    }

    #[test]
    fn test_reset_clears_state() {
        let mut s = default_sampler();
        s.should_sample("port:80");
        s.reset("port:80");
        assert!(s.stats("port:80").is_none());
    }

    #[test]
    fn test_stats_returns_none_for_unknown_key() {
        let s = default_sampler();
        assert!(s.stats("port:9999").is_none());
    }
}

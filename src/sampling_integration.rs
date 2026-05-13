//! Integration of the Sampler into the main alert pipeline.
//! Wraps alert emission with adaptive sampling to reduce alert fatigue.

use crate::sampling::{Sampler, SamplingConfig};
use crate::alert_manager::Alert;

pub struct SamplingFilter {
    sampler: Sampler,
}

impl SamplingFilter {
    pub fn new(config: SamplingConfig) -> Self {
        Self {
            sampler: Sampler::new(config),
        }
    }

    /// Returns Some(alert) if it passes sampling, None if dropped.
    pub fn filter(&mut self, alert: Alert) -> Option<Alert> {
        let key = format!("{}:{}", alert.port, alert.kind);
        if self.sampler.should_sample(&key) {
            Some(alert)
        } else {
            None
        }
    }

    /// Process a batch of alerts, returning only sampled ones.
    pub fn filter_batch(&mut self, alerts: Vec<Alert>) -> Vec<Alert> {
        alerts.into_iter().filter_map(|a| self.filter(a)).collect()
    }

    pub fn sampler_stats(&self, port: u16, kind: &str) -> Option<(u32, u32)> {
        let key = format!("{}:{}", port, kind);
        self.sampler.stats(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alert_manager::Alert;
    use std::time::Duration;

    fn make_alert(port: u16, kind: &str) -> Alert {
        Alert { port, kind: kind.to_string(), message: "test".to_string() }
    }

    #[test]
    fn test_filter_passes_low_frequency() {
        let mut f = SamplingFilter::new(SamplingConfig {
            base_rate: 1.0,
            high_freq_threshold: 100,
            reduced_rate: 0.5,
            window: Duration::from_secs(60),
        });
        let result = f.filter(make_alert(80, "opened"));
        assert!(result.is_some());
    }

    #[test]
    fn test_filter_batch_returns_subset_under_high_freq() {
        let mut f = SamplingFilter::new(SamplingConfig {
            base_rate: 1.0,
            high_freq_threshold: 2,
            reduced_rate: 0.0,
            window: Duration::from_secs(60),
        });
        let alerts: Vec<Alert> = (0..10).map(|_| make_alert(22, "opened")).collect();
        let kept = f.filter_batch(alerts);
        // After threshold=2, reduced_rate=0 drops everything
        assert!(kept.len() <= 2);
    }

    #[test]
    fn test_sampler_stats_accessible() {
        let mut f = SamplingFilter::new(SamplingConfig::default());
        f.filter(make_alert(443, "closed"));
        let stats = f.sampler_stats(443, "closed");
        assert!(stats.is_some());
    }
}

//! Adaptive sampling for port scan events.
//! Reduces noise by sampling high-frequency events at a configurable rate.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SamplingConfig {
    /// Base sample rate (0.0 - 1.0). 1.0 = keep all events.
    pub base_rate: f64,
    /// If event count exceeds this threshold within the window, apply reduced rate.
    pub high_freq_threshold: u32,
    /// Reduced rate applied when threshold is exceeded.
    pub reduced_rate: f64,
    /// Window duration for frequency counting.
    pub window: Duration,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            base_rate: 1.0,
            high_freq_threshold: 10,
            reduced_rate: 0.1,
            window: Duration::from_secs(60),
        }
    }
}

#[derive(Debug)]
struct BucketState {
    count: u32,
    window_start: Instant,
    sampled: u32,
}

#[derive(Debug)]
pub struct Sampler {
    config: SamplingConfig,
    buckets: HashMap<String, BucketState>,
}

impl Sampler {
    pub fn new(config: SamplingConfig) -> Self {
        Self {
            config,
            buckets: HashMap::new(),
        }
    }

    /// Returns true if the event with the given key should be kept.
    pub fn should_sample(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let state = self.buckets.entry(key.to_string()).or_insert(BucketState {
            count: 0,
            window_start: now,
            sampled: 0,
        });

        if now.duration_since(state.window_start) >= self.config.window {
            state.count = 0;
            state.sampled = 0;
            state.window_start = now;
        }

        state.count += 1;

        let rate = if state.count > self.config.high_freq_threshold {
            self.config.reduced_rate
        } else {
            self.config.base_rate
        };

        let keep = (state.sampled as f64 / state.count as f64) < rate;
        if keep {
            state.sampled += 1;
        }
        keep
    }

    pub fn stats(&self, key: &str) -> Option<(u32, u32)> {
        self.buckets.get(key).map(|s| (s.count, s.sampled))
    }

    pub fn reset(&mut self, key: &str) {
        self.buckets.remove(key);
    }
}

//! Exponential backoff strategy for retry logic in notification and webhook dispatch.

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BackoffConfig {
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub max_attempts: u32,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            initial_delay_ms: 250,
            max_delay_ms: 30_000,
            multiplier: 2.0,
            max_attempts: 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Backoff {
    config: BackoffConfig,
    attempt: u32,
}

impl Backoff {
    pub fn new(config: BackoffConfig) -> Self {
        Self { config, attempt: 0 }
    }

    pub fn with_defaults() -> Self {
        Self::new(BackoffConfig::default())
    }

    /// Returns the next delay duration, or None if max attempts exceeded.
    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.attempt >= self.config.max_attempts {
            return None;
        }
        let delay_ms = if self.attempt == 0 {
            self.config.initial_delay_ms
        } else {
            let exp = self.config.multiplier.powi(self.attempt as i32);
            let ms = (self.config.initial_delay_ms as f64 * exp) as u64;
            ms.min(self.config.max_delay_ms)
        };
        self.attempt += 1;
        Some(Duration::from_millis(delay_ms))
    }

    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    pub fn exhausted(&self) -> bool {
        self.attempt >= self.config.max_attempts
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
    }
}

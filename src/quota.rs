//! Quota management for notification channels.
//! Tracks per-channel usage and enforces configurable limits.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct QuotaConfig {
    pub max_per_window: usize,
    pub window: Duration,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            max_per_window: 100,
            window: Duration::from_secs(3600),
        }
    }
}

#[derive(Debug)]
struct ChannelBucket {
    count: usize,
    window_start: Instant,
}

#[derive(Debug, Default)]
pub struct QuotaManager {
    buckets: HashMap<String, ChannelBucket>,
    config: HashMap<String, QuotaConfig>,
    global: QuotaConfig,
}

impl QuotaManager {
    pub fn new(global: QuotaConfig) -> Self {
        Self {
            global,
            ..Default::default()
        }
    }

    pub fn set_channel_config(&mut self, channel: &str, config: QuotaConfig) {
        self.config.insert(channel.to_string(), config);
    }

    fn effective_config(&self, channel: &str) -> &QuotaConfig {
        self.config.get(channel).unwrap_or(&self.global)
    }

    /// Returns true if the channel is within quota and increments the counter.
    pub fn check_and_consume(&mut self, channel: &str) -> bool {
        let cfg = self.effective_config(channel).clone();
        let now = Instant::now();
        let bucket = self.buckets.entry(channel.to_string()).or_insert(ChannelBucket {
            count: 0,
            window_start: now,
        });
        if now.duration_since(bucket.window_start) >= cfg.window {
            bucket.count = 0;
            bucket.window_start = now;
        }
        if bucket.count < cfg.max_per_window {
            bucket.count += 1;
            true
        } else {
            false
        }
    }

    pub fn remaining(&self, channel: &str) -> usize {
        let cfg = self.effective_config(channel);
        match self.buckets.get(channel) {
            None => cfg.max_per_window,
            Some(b) => cfg.max_per_window.saturating_sub(b.count),
        }
    }

    pub fn reset(&mut self, channel: &str) {
        self.buckets.remove(channel);
    }
}

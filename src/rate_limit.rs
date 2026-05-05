//! Rate limiting for notification dispatch to prevent alert flooding.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks per-channel rate limiting state.
pub struct RateLimiter {
    /// Maximum number of notifications allowed per window.
    max_per_window: usize,
    /// Duration of the sliding window.
    window: Duration,
    /// Map from channel key to list of timestamps within the current window.
    history: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    /// Create a new `RateLimiter`.
    ///
    /// # Arguments
    /// * `max_per_window` - Maximum notifications allowed within `window`.
    /// * `window` - Duration of the rate-limiting window.
    pub fn new(max_per_window: usize, window: Duration) -> Self {
        Self {
            max_per_window,
            window,
            history: HashMap::new(),
        }
    }

    /// Returns `true` if a notification for `channel` is allowed right now,
    /// and records the attempt. Returns `false` if the rate limit is exceeded.
    pub fn allow(&mut self, channel: &str) -> bool {
        let now = Instant::now();
        let cutoff = now - self.window;

        let timestamps = self.history.entry(channel.to_string()).or_default();

        // Evict entries outside the window.
        timestamps.retain(|&t| t > cutoff);

        if timestamps.len() < self.max_per_window {
            timestamps.push(now);
            true
        } else {
            false
        }
    }

    /// Returns the number of remaining allowed notifications for `channel`
    /// within the current window.
    pub fn remaining(&self, channel: &str) -> usize {
        let now = Instant::now();
        let cutoff = now - self.window;

        let used = self
            .history
            .get(channel)
            .map(|ts| ts.iter().filter(|&&t| t > cutoff).count())
            .unwrap_or(0);

        self.max_per_window.saturating_sub(used)
    }

    /// Resets rate-limit history for a specific channel.
    pub fn reset(&mut self, channel: &str) {
        self.history.remove(channel);
    }
}

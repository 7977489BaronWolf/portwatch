//! Deduplication module for suppressing repeated identical alerts
//! within a configurable time window.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks the last time an alert was emitted for a given key.
pub struct DedupCache {
    window: Duration,
    last_seen: HashMap<String, Instant>,
}

impl DedupCache {
    /// Create a new DedupCache with the given dedup window.
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_seen: HashMap::new(),
        }
    }

    /// Returns `true` if the event with `key` is a duplicate and should
    /// be suppressed.  Returns `false` (and records the event) if it
    /// should be forwarded.
    pub fn is_duplicate(&mut self, key: &str) -> bool {
        let now = Instant::now();
        if let Some(&last) = self.last_seen.get(key) {
            if now.duration_since(last) < self.window {
                return true;
            }
        }
        self.last_seen.insert(key.to_string(), now);
        false
    }

    /// Remove entries whose last-seen timestamp has expired, keeping
    /// memory usage bounded during long daemon runs.
    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.last_seen
            .retain(|_, last| now.duration_since(*last) < self.window);
    }

    /// Number of keys currently tracked.
    pub fn len(&self) -> usize {
        self.last_seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.last_seen.is_empty()
    }
}

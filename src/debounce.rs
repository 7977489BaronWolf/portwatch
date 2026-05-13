//! Debounce module: suppresses repeated alerts within a configurable quiet window.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tracks the last emission time for each alert key.
pub struct Debouncer {
    window: Duration,
    last_seen: HashMap<String, Instant>,
}

impl Debouncer {
    /// Create a new `Debouncer` with the given quiet window.
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_seen: HashMap::new(),
        }
    }

    /// Returns `true` if the alert for `key` should be emitted now.
    /// Subsequent calls within `window` will return `false`.
    pub fn should_emit(&mut self, key: &str) -> bool {
        let now = Instant::now();
        match self.last_seen.get(key) {
            Some(&last) if now.duration_since(last) < self.window => false,
            _ => {
                self.last_seen.insert(key.to_string(), now);
                true
            }
        }
    }

    /// Force-reset the debounce state for a specific key.
    pub fn reset(&mut self, key: &str) {
        self.last_seen.remove(key);
    }

    /// Clear all tracked keys.
    pub fn clear(&mut self) {
        self.last_seen.clear();
    }

    /// Returns the number of keys currently being tracked.
    pub fn tracked_count(&self) -> usize {
        self.last_seen.len()
    }
}

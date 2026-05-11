//! Cooldown tracking: prevents repeated alerts for the same port/event
//! within a configurable cooldown window.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CooldownTracker {
    window: Duration,
    last_seen: HashMap<String, Instant>,
}

impl CooldownTracker {
    /// Create a new tracker with the given cooldown window.
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            last_seen: HashMap::new(),
        }
    }

    /// Returns `true` if the key is NOT in cooldown (i.e. alert should fire).
    /// Updates the last-seen timestamp when returning `true`.
    pub fn allow(&mut self, key: &str) -> bool {
        let now = Instant::now();
        if let Some(&last) = self.last_seen.get(key) {
            if now.duration_since(last) < self.window {
                return false;
            }
        }
        self.last_seen.insert(key.to_string(), now);
        true
    }

    /// Explicitly reset the cooldown for a key, allowing the next call to
    /// `allow` to return `true` immediately.
    pub fn reset(&mut self, key: &str) {
        self.last_seen.remove(key);
    }

    /// Remove all entries whose cooldown window has already elapsed.
    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.last_seen
            .retain(|_, last| now.duration_since(*last) < self.window);
    }

    /// Number of keys currently tracked (including those still in cooldown).
    pub fn len(&self) -> usize {
        self.last_seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.last_seen.is_empty()
    }
}

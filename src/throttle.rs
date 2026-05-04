//! Throttle module: prevents alert flooding by rate-limiting notifications
//! per port/event-type combination.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Key identifying a unique alert stream (port + event kind).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrottleKey {
    pub port: u16,
    pub event_kind: String,
}

impl ThrottleKey {
    pub fn new(port: u16, event_kind: impl Into<String>) -> Self {
        Self { port, event_kind: event_kind.into() }
    }
}

/// Tracks last-fired timestamps and enforces a minimum interval between alerts.
pub struct Throttle {
    /// Minimum time between two alerts for the same key.
    cooldown: Duration,
    last_fired: HashMap<ThrottleKey, Instant>,
}

impl Throttle {
    /// Create a new `Throttle` with the given cooldown period.
    pub fn new(cooldown: Duration) -> Self {
        Self {
            cooldown,
            last_fired: HashMap::new(),
        }
    }

    /// Returns `true` if the alert for `key` is allowed to fire right now.
    /// Records the current time as the last-fired instant when allowed.
    pub fn allow(&mut self, key: &ThrottleKey) -> bool {
        let now = Instant::now();
        match self.last_fired.get(key) {
            Some(&last) if now.duration_since(last) < self.cooldown => false,
            _ => {
                self.last_fired.insert(key.clone(), now);
                true
            }
        }
    }

    /// Clears the recorded state for a given key (e.g. after a baseline reset).
    pub fn reset(&mut self, key: &ThrottleKey) {
        self.last_fired.remove(key);
    }

    /// Clears all throttle state.
    pub fn reset_all(&mut self) {
        self.last_fired.clear();
    }

    /// Returns the remaining cooldown for a key, or `None` if it may fire now.
    pub fn remaining(&self, key: &ThrottleKey) -> Option<Duration> {
        self.last_fired.get(key).and_then(|&last| {
            let elapsed = Instant::now().duration_since(last);
            if elapsed < self.cooldown {
                Some(self.cooldown - elapsed)
            } else {
                None
            }
        })
    }
}

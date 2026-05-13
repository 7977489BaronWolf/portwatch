//! Alert aggregation: groups related alerts within a time window to reduce noise.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct AggregatedGroup {
    pub key: String,
    pub count: usize,
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub sample: String,
}

#[derive(Debug)]
pub struct Aggregator {
    window: Duration,
    groups: HashMap<String, AggregatedGroup>,
}

impl Aggregator {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            groups: HashMap::new(),
        }
    }

    /// Submit an event with a grouping key and a sample description.
    /// Returns Some(group) if the group is new (first occurrence in window).
    pub fn submit(&mut self, key: &str, sample: &str) -> Option<&AggregatedGroup> {
        let now = Instant::now();
        self.evict_expired(now);

        if let Some(group) = self.groups.get_mut(key) {
            group.count += 1;
            group.last_seen = now;
            None
        } else {
            let group = AggregatedGroup {
                key: key.to_string(),
                count: 1,
                first_seen: now,
                last_seen: now,
                sample: sample.to_string(),
            };
            self.groups.insert(key.to_string(), group);
            self.groups.get(key)
        }
    }

    /// Returns all active groups.
    pub fn active_groups(&self) -> Vec<&AggregatedGroup> {
        self.groups.values().collect()
    }

    /// Flush and return all groups, clearing internal state.
    pub fn flush(&mut self) -> Vec<AggregatedGroup> {
        self.groups.drain().map(|(_, v)| v).collect()
    }

    fn evict_expired(&mut self, now: Instant) {
        self.groups
            .retain(|_, g| now.duration_since(g.last_seen) < self.window);
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

use crate::priority::Priority;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityItem {
    pub id: String,
    pub priority: Priority,
    pub payload: String,
    pub enqueued_at: u64,
}

impl PriorityItem {
    pub fn new(id: impl Into<String>, priority: Priority, payload: impl Into<String>) -> Self {
        let enqueued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            id: id.into(),
            priority,
            payload: payload.into(),
            enqueued_at,
        }
    }
}

#[derive(Debug, Default)]
pub struct PriorityQueue {
    buckets: BTreeMap<u8, Vec<PriorityItem>>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self::default()
    }

    fn bucket_key(priority: &Priority) -> u8 {
        // Higher priority = lower key so BTreeMap pops highest first
        match priority {
            Priority::Critical => 0,
            Priority::High => 1,
            Priority::Medium => 2,
            Priority::Low => 3,
        }
    }

    pub fn push(&mut self, item: PriorityItem) {
        let key = Self::bucket_key(&item.priority);
        self.buckets.entry(key).or_default().push(item);
    }

    pub fn pop(&mut self) -> Option<PriorityItem> {
        if let Some((&key, _)) = self.buckets.iter().next() {
            if let Some(bucket) = self.buckets.get_mut(&key) {
                if !bucket.is_empty() {
                    let item = bucket.remove(0);
                    if bucket.is_empty() {
                        self.buckets.remove(&key);
                    }
                    return Some(item);
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.buckets.values().map(|v| v.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn drain_by_priority(&mut self, priority: &Priority) -> Vec<PriorityItem> {
        let key = Self::bucket_key(priority);
        self.buckets.remove(&key).unwrap_or_default()
    }

    pub fn peek(&self) -> Option<&PriorityItem> {
        self.buckets.values().next().and_then(|b| b.first())
    }
}

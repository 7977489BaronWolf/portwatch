//! Port event grouping — clusters related port change events by protocol, subnet, or time window.

use std::collections::HashMap;
use crate::diff_engine::PortChange;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupKey {
    Protocol(String),
    Subnet(String),
    TimeWindow(u64), // epoch-based bucket
}

#[derive(Debug, Clone)]
pub struct PortGroup {
    pub key: GroupKey,
    pub changes: Vec<PortChange>,
}

impl PortGroup {
    pub fn new(key: GroupKey) -> Self {
        Self { key, changes: Vec::new() }
    }

    pub fn add(&mut self, change: PortChange) {
        self.changes.push(change);
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

/// Groups a slice of `PortChange` events by protocol.
pub fn group_by_protocol(changes: &[PortChange]) -> Vec<PortGroup> {
    let mut map: HashMap<String, PortGroup> = HashMap::new();
    for change in changes {
        let proto = change.protocol.clone();
        map.entry(proto.clone())
            .or_insert_with(|| PortGroup::new(GroupKey::Protocol(proto)))
            .add(change.clone());
    }
    map.into_values().collect()
}

/// Groups a slice of `PortChange` events into fixed-size time buckets (bucket_secs width).
pub fn group_by_time_window(changes: &[PortChange], bucket_secs: u64) -> Vec<PortGroup> {
    if bucket_secs == 0 {
        return Vec::new();
    }
    let mut map: HashMap<u64, PortGroup> = HashMap::new();
    for change in changes {
        let bucket = change.timestamp / bucket_secs;
        map.entry(bucket)
            .or_insert_with(|| PortGroup::new(GroupKey::TimeWindow(bucket * bucket_secs)))
            .add(change.clone());
    }
    let mut groups: Vec<PortGroup> = map.into_values().collect();
    groups.sort_by_key(|g| match &g.key {
        GroupKey::TimeWindow(t) => *t,
        _ => 0,
    });
    groups
}

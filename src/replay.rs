//! Replay module: replays historical port scan events from audit log or snapshots
//! for debugging, testing notification hooks, or re-evaluating alerts.

use crate::audit_log::AuditEntry;
use crate::snapshot::Snapshot;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum ReplaySource {
    AuditLog,
    Snapshot,
}

#[derive(Debug, Clone)]
pub struct ReplayEvent {
    pub timestamp: u64,
    pub source: ReplaySource,
    pub description: String,
    pub port: u16,
    pub protocol: String,
}

impl ReplayEvent {
    pub fn from_audit(entry: &AuditEntry) -> Self {
        ReplayEvent {
            timestamp: entry.timestamp,
            source: ReplaySource::AuditLog,
            description: entry.message.clone(),
            port: entry.port,
            protocol: entry.protocol.clone(),
        }
    }

    pub fn from_snapshot(snap: &Snapshot, port: u16, protocol: &str) -> Self {
        ReplayEvent {
            timestamp: snap.created_at,
            source: ReplaySource::Snapshot,
            description: format!("Snapshot '{}' port {}/{}", snap.name, port, protocol),
            port,
            protocol: protocol.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ReplayBuffer {
    events: VecDeque<ReplayEvent>,
}

impl ReplayBuffer {
    pub fn new() -> Self {
        ReplayBuffer {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: ReplayEvent) {
        self.events.push_back(event);
    }

    pub fn next(&mut self) -> Option<ReplayEvent> {
        self.events.pop_front()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn drain_all(&mut self) -> Vec<ReplayEvent> {
        self.events.drain(..).collect()
    }

    pub fn filter_by_port(&self, port: u16) -> Vec<&ReplayEvent> {
        self.events.iter().filter(|e| e.port == port).collect()
    }

    pub fn filter_by_source(&self, source: &ReplaySource) -> Vec<&ReplayEvent> {
        self.events.iter().filter(|e| &e.source == source).collect()
    }
}

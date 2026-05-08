//! Timeline module: tracks port change events over time for historical analysis.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Maximum number of events retained in the timeline by default.
const DEFAULT_MAX_EVENTS: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimelineEventKind {
    PortOpened,
    PortClosed,
    PortChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: u64,
    pub port: u16,
    pub protocol: String,
    pub kind: TimelineEventKind,
    pub description: String,
}

impl TimelineEvent {
    pub fn new(port: u16, protocol: &str, kind: TimelineEventKind, description: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            timestamp,
            port,
            protocol: protocol.to_string(),
            kind,
            description: description.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Timeline {
    events: VecDeque<TimelineEvent>,
    max_events: usize,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: DEFAULT_MAX_EVENTS,
        }
    }

    pub fn with_capacity(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }

    pub fn push(&mut self, event: TimelineEvent) {
        if self.events.len() >= self.max_events {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn events(&self) -> &VecDeque<TimelineEvent> {
        &self.events
    }

    pub fn events_for_port(&self, port: u16) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| e.port == port).collect()
    }

    pub fn since(&self, since_ts: u64) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| e.timestamp >= since_ts).collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

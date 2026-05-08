//! Rollup module: aggregates multiple port change events into a single
//! summarized report over a configurable time window.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeKind {
    Opened,
    Closed,
}

#[derive(Debug, Clone)]
pub struct PortEvent {
    pub port: u16,
    pub kind: ChangeKind,
    pub timestamp: Instant,
}

impl PortEvent {
    pub fn new(port: u16, kind: ChangeKind) -> Self {
        Self {
            port,
            kind,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RollupSummary {
    pub opened: Vec<u16>,
    pub closed: Vec<u16>,
    pub event_count: usize,
}

impl RollupSummary {
    pub fn is_empty(&self) -> bool {
        self.opened.is_empty() && self.closed.is_empty()
    }

    pub fn format(&self) -> String {
        format!(
            "Rollup: {} event(s) — opened: {:?}, closed: {:?}",
            self.event_count, self.opened, self.closed
        )
    }
}

pub struct RollupBuffer {
    window: Duration,
    events: Vec<PortEvent>,
}

impl RollupBuffer {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: PortEvent) {
        self.events.push(event);
    }

    /// Drain events within the window and return a summary.
    pub fn flush(&mut self) -> RollupSummary {
        let cutoff = Instant::now() - self.window;
        let (recent, _expired): (Vec<_>, Vec<_>) =
            self.events.drain(..).partition(|e| e.timestamp >= cutoff);

        let event_count = recent.len();
        let mut tally: HashMap<u16, ChangeKind> = HashMap::new();
        for e in recent {
            tally.insert(e.port, e.kind);
        }

        let mut summary = RollupSummary {
            event_count,
            ..Default::default()
        };
        for (port, kind) in tally {
            match kind {
                ChangeKind::Opened => summary.opened.push(port),
                ChangeKind::Closed => summary.closed.push(port),
            }
        }
        summary.opened.sort_unstable();
        summary.closed.sort_unstable();
        summary
    }
}

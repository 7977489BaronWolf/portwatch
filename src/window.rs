//! Time-window aggregation for port change events.
//!
//! Groups events into fixed-duration windows for batch analysis.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub port: u16,
    pub kind: String,
    pub timestamp: Instant,
}

#[derive(Debug)]
pub struct TimeWindow {
    pub duration: Duration,
    events: VecDeque<WindowEvent>,
}

impl TimeWindow {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            events: VecDeque::new(),
        }
    }

    /// Insert a new event into the window.
    pub fn push(&mut self, event: WindowEvent) {
        self.events.push_back(event);
        self.evict();
    }

    /// Remove events older than the window duration.
    fn evict(&mut self) {
        let cutoff = Instant::now() - self.duration;
        while let Some(front) = self.events.front() {
            if front.timestamp < cutoff {
                self.events.pop_front();
            } else {
                break;
            }
        }
    }

    /// Return a snapshot of all events currently in the window.
    pub fn events(&mut self) -> Vec<&WindowEvent> {
        self.evict();
        self.events.iter().collect()
    }

    /// Count events in the current window.
    pub fn count(&mut self) -> usize {
        self.evict();
        self.events.len()
    }

    /// Count events matching a specific kind.
    pub fn count_by_kind(&mut self, kind: &str) -> usize {
        self.evict();
        self.events.iter().filter(|e| e.kind == kind).count()
    }

    /// Drain and return all events, clearing the window.
    pub fn drain(&mut self) -> Vec<WindowEvent> {
        self.evict();
        self.events.drain(..).collect()
    }

    pub fn is_empty(&mut self) -> bool {
        self.count() == 0
    }
}

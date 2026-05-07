//! Trend analysis for port activity over time windows.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Rising,
    Falling,
    Stable,
}

#[derive(Debug, Clone)]
pub struct TrendPoint {
    pub timestamp: SystemTime,
    pub port_count: usize,
}

#[derive(Debug, Clone)]
pub struct TrendSummary {
    pub direction: TrendDirection,
    pub delta: i64,
    pub window_secs: u64,
    pub samples: usize,
}

#[derive(Debug, Default)]
pub struct TrendTracker {
    history: Vec<TrendPoint>,
    max_samples: usize,
}

impl TrendTracker {
    pub fn new(max_samples: usize) -> Self {
        Self {
            history: Vec::new(),
            max_samples: max_samples.max(2),
        }
    }

    pub fn record(&mut self, port_count: usize) {
        self.history.push(TrendPoint {
            timestamp: SystemTime::now(),
            port_count,
        });
        if self.history.len() > self.max_samples {
            self.history.remove(0);
        }
    }

    pub fn analyze(&self, window: Duration) -> Option<TrendSummary> {
        if self.history.len() < 2 {
            return None;
        }
        let cutoff = SystemTime::now().checked_sub(window)?;
        let windowed: Vec<&TrendPoint> = self
            .history
            .iter()
            .filter(|p| p.timestamp >= cutoff)
            .collect();
        if windowed.len() < 2 {
            return None;
        }
        let first = windowed.first().unwrap().port_count as i64;
        let last = windowed.last().unwrap().port_count as i64;
        let delta = last - first;
        let direction = match delta {
            d if d > 0 => TrendDirection::Rising,
            d if d < 0 => TrendDirection::Falling,
            _ => TrendDirection::Stable,
        };
        Some(TrendSummary {
            direction,
            delta,
            window_secs: window.as_secs(),
            samples: windowed.len(),
        })
    }

    pub fn per_protocol_trend(&self) -> HashMap<String, i64> {
        HashMap::new()
    }

    pub fn history(&self) -> &[TrendPoint] {
        &self.history
    }
}

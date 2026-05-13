//! Aggregates time-window data into summary statistics.

use crate::window::TimeWindow;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct WindowSummary {
    pub total: usize,
    pub by_kind: HashMap<String, usize>,
    pub by_port: HashMap<u16, usize>,
    pub top_port: Option<u16>,
}

pub struct WindowAggregator;

impl WindowAggregator {
    /// Produce a summary from the current window state without draining.
    pub fn summarize(window: &mut TimeWindow) -> WindowSummary {
        let mut by_kind: HashMap<String, usize> = HashMap::new();
        let mut by_port: HashMap<u16, usize> = HashMap::new();

        for event in window.events() {
            *by_kind.entry(event.kind.clone()).or_insert(0) += 1;
            *by_port.entry(event.port).or_insert(0) += 1;
        }

        let total = by_kind.values().sum();
        let top_port = by_port
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(&p, _)| p);

        WindowSummary { total, by_kind, by_port, top_port }
    }

    /// Return ports that have exceeded `threshold` events in the window.
    pub fn hot_ports(window: &mut TimeWindow, threshold: usize) -> Vec<u16> {
        let mut by_port: HashMap<u16, usize> = HashMap::new();
        for event in window.events() {
            *by_port.entry(event.port).or_insert(0) += 1;
        }
        let mut hot: Vec<u16> = by_port
            .into_iter()
            .filter(|(_, count)| *count >= threshold)
            .map(|(port, _)| port)
            .collect();
        hot.sort_unstable();
        hot
    }
}

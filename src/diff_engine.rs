use std::collections::HashSet;

/// Represents a single listening port entry captured during a scan.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortEntry {
    pub port: u16,
    pub protocol: String,
    pub pid: Option<u32>,
    pub process: Option<String>,
}

impl PortEntry {
    pub fn key(&self) -> String {
        format!("{}:{}", self.protocol, self.port)
    }
}

/// Describes a change between two port snapshots.
#[derive(Debug, Clone)]
pub enum PortDiff {
    Opened(PortEntry),
    Closed(PortEntry),
}

impl PortDiff {
    pub fn key(&self) -> String {
        match self {
            PortDiff::Opened(e) | PortDiff::Closed(e) => e.key(),
        }
    }
}

/// Compute the diff between two port snapshots.
pub fn compute_diff(previous: &[PortEntry], current: &[PortEntry]) -> Vec<PortDiff> {
    let prev_set: HashSet<_> = previous.iter().collect();
    let curr_set: HashSet<_> = current.iter().collect();

    let mut diffs = Vec::new();

    for entry in curr_set.difference(&prev_set) {
        diffs.push(PortDiff::Opened((*entry).clone()));
    }

    for entry in prev_set.difference(&curr_set) {
        diffs.push(PortDiff::Closed((*entry).clone()));
    }

    diffs
}

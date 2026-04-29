use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct PortDiff {
    pub opened: Vec<u16>,
    pub closed: Vec<u16>,
}

impl PortDiff {
    pub fn is_empty(&self) -> bool {
        self.opened.is_empty() && self.closed.is_empty()
    }
}

pub fn compute_diff(previous: &[u16], current: &[u16]) -> PortDiff {
    let prev_set: HashSet<u16> = previous.iter().cloned().collect();
    let curr_set: HashSet<u16> = current.iter().cloned().collect();

    let mut opened: Vec<u16> = curr_set.difference(&prev_set).cloned().collect();
    let mut closed: Vec<u16> = prev_set.difference(&curr_set).cloned().collect();

    opened.sort_unstable();
    closed.sort_unstable();

    PortDiff { opened, closed }
}

pub fn format_diff_message(diff: &PortDiff) -> String {
    let mut parts = Vec::new();

    if !diff.opened.is_empty() {
        let ports: Vec<String> = diff.opened.iter().map(|p| p.to_string()).collect();
        parts.push(format!("Opened ports: {}", ports.join(", ")));
    }

    if !diff.closed.is_empty() {
        let ports: Vec<String> = diff.closed.iter().map(|p| p.to_string()).collect();
        parts.push(format!("Closed ports: {}", ports.join(", ")));
    }

    if parts.is_empty() {
        return "No port changes detected.".to_string();
    }

    parts.join(" | ")
}

#[cfg(test)]
#[path = "diff_engine_tests.rs"]
mod tests;

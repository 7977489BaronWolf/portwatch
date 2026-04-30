use crate::diff_engine::PortDiff;
use crate::state_store::PortState;
use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Report {
    pub generated_at: DateTime<Local>,
    pub hostname: String,
    pub current_state: PortState,
    pub changes: Vec<PortDiff>,
    pub total_open: usize,
}

impl Report {
    pub fn new(current_state: PortState, changes: Vec<PortDiff>) -> Self {
        let total_open = current_state.ports.len();
        Report {
            generated_at: Local::now(),
            hostname: hostname::get()
                .map(|h| h.to_string_lossy().into_owned())
                .unwrap_or_else(|_| "unknown".to_string()),
            current_state,
            changes,
            total_open,
        }
    }

    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn to_summary(&self) -> String {
        if self.changes.is_empty() {
            return format!(
                "[{}] No port changes detected on {}. {} ports open.",
                self.generated_at.format("%Y-%m-%d %H:%M:%S"),
                self.hostname,
                self.total_open
            );
        }
        format!(
            "[{}] {} port change(s) detected on {}. {} ports currently open.",
            self.generated_at.format("%Y-%m-%d %H:%M:%S"),
            self.changes.len(),
            self.hostname,
            self.total_open
        )
    }

    pub fn to_detailed(&self) -> String {
        let mut out = self.to_summary();
        if !self.changes.is_empty() {
            out.push_str("\nChanges:\n");
            for diff in &self.changes {
                out.push_str(&format!("  {}", diff));
                out.push('\n');
            }
        }
        out
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_detailed())
    }
}

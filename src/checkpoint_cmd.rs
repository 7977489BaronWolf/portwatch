//! CLI command handlers for checkpoint management.

use crate::checkpoint::{Checkpoint, CheckpointStore};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CheckpointCmd {
    Create { label: String },
    List,
    Show { label: String },
    Clear,
}

pub struct CheckpointCmdHandler {
    store: CheckpointStore,
    sequence: u64,
}

impl CheckpointCmdHandler {
    pub fn new(max_retained: usize) -> Self {
        Self {
            store: CheckpointStore::new(max_retained),
            sequence: 0,
        }
    }

    pub fn handle(&mut self, cmd: CheckpointCmd, current_ports: &HashMap<u16, String>) -> String {
        match cmd {
            CheckpointCmd::Create { label } => {
                self.sequence += 1;
                let cp = Checkpoint::new(label.clone(), current_ports.clone(), self.sequence);
                let port_count = cp.port_count();
                self.store.save(cp);
                format!("Checkpoint '{}' created (seq={}, ports={})", label, self.sequence, port_count)
            }
            CheckpointCmd::List => {
                let all = self.store.all();
                if all.is_empty() {
                    return "No checkpoints stored.".to_string();
                }
                let lines: Vec<String> = all
                    .iter()
                    .map(|c| format!("  [{}] label='{}' ports={} age={}s", c.sequence, c.label, c.port_count(), c.age_secs()))
                    .collect();
                format!("Checkpoints ({}):\n{}", all.len(), lines.join("\n"))
            }
            CheckpointCmd::Show { label } => {
                match self.store.find_by_label(&label) {
                    Some(cp) => format!(
                        "Checkpoint '{}':\n  seq={}\n  ports={}\n  age={}s",
                        cp.label, cp.sequence, cp.port_count(), cp.age_secs()
                    ),
                    None => format!("No checkpoint found with label '{}'.", label),
                }
            }
            CheckpointCmd::Clear => {
                self.store.clear();
                self.sequence = 0;
                "All checkpoints cleared.".to_string()
            }
        }
    }

    pub fn store(&self) -> &CheckpointStore {
        &self.store
    }
}

//! Checkpoint module: periodic state snapshots with recovery support.

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checkpoint {
    pub id: u64,
    pub timestamp: u64,
    pub label: String,
    pub port_state: HashMap<u16, String>,
    pub sequence: u64,
}

impl Checkpoint {
    pub fn new(label: impl Into<String>, port_state: HashMap<u16, String>, sequence: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let id = timestamp ^ sequence;
        Self {
            id,
            timestamp,
            label: label.into(),
            port_state,
            sequence,
        }
    }

    pub fn age_secs(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.timestamp)
    }

    pub fn port_count(&self) -> usize {
        self.port_state.len()
    }
}

#[derive(Debug, Default)]
pub struct CheckpointStore {
    checkpoints: Vec<Checkpoint>,
    max_retained: usize,
}

impl CheckpointStore {
    pub fn new(max_retained: usize) -> Self {
        Self {
            checkpoints: Vec::new(),
            max_retained: max_retained.max(1),
        }
    }

    pub fn save(&mut self, cp: Checkpoint) {
        self.checkpoints.push(cp);
        if self.checkpoints.len() > self.max_retained {
            self.checkpoints.remove(0);
        }
    }

    pub fn latest(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }

    pub fn find_by_label(&self, label: &str) -> Option<&Checkpoint> {
        self.checkpoints.iter().rev().find(|c| c.label == label)
    }

    pub fn all(&self) -> &[Checkpoint] {
        &self.checkpoints
    }

    pub fn clear(&mut self) {
        self.checkpoints.clear();
    }
}

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortState {
    pub ports: HashSet<u16>,
    pub timestamp: u64,
}

impl PortState {
    pub fn new(ports: HashSet<u16>, timestamp: u64) -> Self {
        Self { ports, timestamp }
    }

    pub fn diff(&self, other: &PortState) -> PortDiff {
        let opened: HashSet<u16> = other.ports.difference(&self.ports).copied().collect();
        let closed: HashSet<u16> = self.ports.difference(&other.ports).copied().collect();
        PortDiff { opened, closed }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PortDiff {
    pub opened: HashSet<u16>,
    pub closed: HashSet<u16>,
}

impl PortDiff {
    pub fn is_empty(&self) -> bool {
        self.opened.is_empty() && self.closed.is_empty()
    }
}

pub struct StateStore {
    path: PathBuf,
}

impl StateStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn load(&self) -> io::Result<Option<PortState>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(&self.path)?;
        let state: PortState = serde_json::from_str(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Some(state))
    }

    pub fn save(&self, state: &PortState) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(state)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.path, data)
    }
}

//! Snapshot module: captures and persists point-in-time port state snapshots.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::port_scanner::PortEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: u64,
    pub timestamp: u64,
    pub label: Option<String>,
    pub ports: Vec<PortEntry>,
}

impl Snapshot {
    pub fn new(ports: Vec<PortEntry>, label: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let id = timestamp;
        Self { id, timestamp, label, ports }
    }

    pub fn port_map(&self) -> HashMap<u16, &PortEntry> {
        self.ports.iter().map(|p| (p.port, p)).collect()
    }
}

pub struct SnapshotStore {
    dir: PathBuf,
}

impl SnapshotStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        fs::create_dir_all(&dir).ok();
        Self { dir }
    }

    fn path_for(&self, id: u64) -> PathBuf {
        self.dir.join(format!("snapshot_{}.json", id))
    }

    pub fn save(&self, snapshot: &Snapshot) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(snapshot)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(self.path_for(snapshot.id), data)
    }

    pub fn load(&self, id: u64) -> std::io::Result<Snapshot> {
        let data = fs::read_to_string(self.path_for(id))?;
        serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn list(&self) -> std::io::Result<Vec<Snapshot>> {
        let mut snapshots = Vec::new();
        for entry in fs::read_dir(&self.dir)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("snapshot_") && name.ends_with(".json") {
                if let Ok(data) = fs::read_to_string(entry.path()) {
                    if let Ok(snap) = serde_json::from_str::<Snapshot>(&data) {
                        snapshots.push(snap);
                    }
                }
            }
        }
        snapshots.sort_by_key(|s| s.timestamp);
        Ok(snapshots)
    }

    pub fn delete(&self, id: u64) -> std::io::Result<()> {
        let p = self.path_for(id);
        if p.exists() { fs::remove_file(p) } else { Ok(()) }
    }

    pub fn latest(&self) -> std::io::Result<Option<Snapshot>> {
        Ok(self.list()?.into_iter().last())
    }
}

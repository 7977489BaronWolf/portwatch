use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub event_type: AuditEventType,
    pub description: String,
    pub port: Option<u16>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    PortOpened,
    PortClosed,
    BaselineUpdated,
    FilterChanged,
    DaemonStarted,
    DaemonStopped,
    AlertFired,
}

#[derive(Debug)]
pub struct AuditLog {
    path: PathBuf,
}

impl AuditLog {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(Self { path })
    }

    pub fn record(&self, event_type: AuditEventType, description: &str, port: Option<u16>, protocol: Option<&str>) -> io::Result<()> {
        let entry = AuditEntry {
            timestamp: Utc::now().to_rfc3339(),
            event_type,
            description: description.to_string(),
            port,
            protocol: protocol.map(String::from),
        };
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let line = serde_json::to_string(&entry)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(file, "{}", line)
    }

    pub fn read_entries(&self) -> io::Result<Vec<AuditEntry>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let content = std::fs::read_to_string(&self.path)?;
        let entries = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        Ok(entries)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

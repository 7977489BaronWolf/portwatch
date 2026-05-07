//! Persistent store for correlation incidents using JSON lines.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use crate::correlation::Incident;

pub struct CorrelationStore {
    path: PathBuf,
}

impl CorrelationStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn append(&self, incident: &Incident) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let line = format!(
            "{{\"id\":\"{}\",\"group\":\"{:?}\",\"changes\":{},\"summary\":\"{}\"}}",
            incident.id,
            incident.group,
            incident.diffs.len(),
            incident.summary
        );
        writeln!(file, "{}", line)
    }

    pub fn load_ids(&self) -> std::io::Result<Vec<String>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let ids = reader
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| extract_id(&l))
            .collect();
        Ok(ids)
    }

    pub fn count(&self) -> usize {
        self.load_ids().unwrap_or_default().len()
    }
}

fn extract_id(line: &str) -> Option<String> {
    let key = "\"id\":\"";
    let start = line.find(key)? + key.len();
    let end = line[start..].find('"')? + start;
    Some(line[start..end].to_string())
}

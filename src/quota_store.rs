//! Persistent storage for quota state across daemon restarts.

use crate::quota::QuotaConfig;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StoredQuotaEntry {
    pub channel: String,
    pub max_per_window: usize,
    pub window_secs: u64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct QuotaStore {
    entries: Vec<StoredQuotaEntry>,
}

impl QuotaStore {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)?;
        let store = serde_json::from_str(&data)?;
        Ok(store)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn upsert(&mut self, channel: &str, config: &QuotaConfig) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.channel == channel) {
            e.max_per_window = config.max_per_window;
            e.window_secs = config.window.as_secs();
        } else {
            self.entries.push(StoredQuotaEntry {
                channel: channel.to_string(),
                max_per_window: config.max_per_window,
                window_secs: config.window.as_secs(),
            });
        }
    }

    pub fn remove(&mut self, channel: &str) {
        self.entries.retain(|e| e.channel != channel);
    }

    pub fn to_configs(&self) -> HashMap<String, QuotaConfig> {
        self.entries
            .iter()
            .map(|e| {
                (
                    e.channel.clone(),
                    QuotaConfig {
                        max_per_window: e.max_per_window,
                        window: Duration::from_secs(e.window_secs),
                    },
                )
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

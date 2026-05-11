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
    /// Load the quota store from a JSON file at `path`.
    /// Returns an empty store if the file does not yet exist.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read quota store at {}: {}", path.display(), e))?;
        let store = serde_json::from_str(&data)
            .map_err(|e| anyhow::anyhow!("Failed to parse quota store at {}: {}", path.display(), e))?;
        Ok(store)
    }

    /// Persist the quota store as pretty-printed JSON to `path`.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)
            .map_err(|e| anyhow::anyhow!("Failed to write quota store to {}: {}", path.display(), e))?;
        Ok(())
    }

    /// Insert or update the quota config for `channel`.
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

    /// Remove the entry for `channel`, if present.
    pub fn remove(&mut self, channel: &str) {
        self.entries.retain(|e| e.channel != channel);
    }

    /// Convert all stored entries into a map of channel name to [`QuotaConfig`].
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

    /// Return the stored config for `channel`, or `None` if not present.
    pub fn get(&self, channel: &str) -> Option<QuotaConfig> {
        self.entries.iter().find(|e| e.channel == channel).map(|e| QuotaConfig {
            max_per_window: e.max_per_window,
            window: Duration::from_secs(e.window_secs),
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

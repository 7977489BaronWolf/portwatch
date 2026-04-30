use std::collections::HashSet;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::port_scanner::PortInfo;
use crate::state_store::StateStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baseline {
    pub ports: HashSet<u16>,
    pub created_at: u64,
    pub label: String,
}

impl Baseline {
    pub fn new(ports: HashSet<u16>, label: impl Into<String>) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            ports,
            created_at,
            label: label.into(),
        }
    }

    pub fn from_port_infos(infos: &[PortInfo], label: impl Into<String>) -> Self {
        let ports = infos.iter().map(|p| p.port).collect();
        Self::new(ports, label)
    }

    pub fn contains(&self, port: u16) -> bool {
        self.ports.contains(&port)
    }

    pub fn unexpected_ports(&self, current: &HashSet<u16>) -> HashSet<u16> {
        current.difference(&self.ports).copied().collect()
    }

    pub fn missing_ports(&self, current: &HashSet<u16>) -> HashSet<u16> {
        self.ports.difference(current).copied().collect()
    }
}

pub struct BaselineManager {
    store: StateStore,
    baseline_key: String,
}

impl BaselineManager {
    pub fn new(store: StateStore) -> Self {
        Self {
            store,
            baseline_key: "baseline".to_string(),
        }
    }

    pub fn save(&self, baseline: &Baseline) -> anyhow::Result<()> {
        let data = serde_json::to_vec(baseline)?;
        self.store.write_raw(&self.baseline_key, &data)
    }

    pub fn load(&self) -> anyhow::Result<Option<Baseline>> {
        match self.store.read_raw(&self.baseline_key)? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.store.delete(&self.baseline_key)
    }
}

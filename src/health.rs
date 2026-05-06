//! Health check module for portwatch daemon.
//!
//! Tracks internal component health and exposes a summary
//! that can be queried via CLI or written to a status file.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Ok,
    Degraded(String),
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub last_checked: SystemTime,
    pub uptime: Duration,
}

impl ComponentHealth {
    pub fn new(name: &str, status: HealthStatus) -> Self {
        Self {
            name: name.to_string(),
            status,
            last_checked: SystemTime::now(),
            uptime: Duration::from_secs(0),
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Ok)
    }
}

#[derive(Debug, Default)]
pub struct HealthRegistry {
    components: HashMap<String, ComponentHealth>,
}

impl HealthRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, health: ComponentHealth) {
        self.components.insert(health.name.clone(), health);
    }

    pub fn update(&mut self, name: &str, status: HealthStatus) {
        if let Some(c) = self.components.get_mut(name) {
            c.status = status;
            c.last_checked = SystemTime::now();
        }
    }

    pub fn overall_status(&self) -> HealthStatus {
        let mut degraded = vec![];
        for c in self.components.values() {
            match &c.status {
                HealthStatus::Failed(msg) => return HealthStatus::Failed(msg.clone()),
                HealthStatus::Degraded(msg) => degraded.push(msg.clone()),
                HealthStatus::Ok => {}
            }
        }
        if degraded.is_empty() {
            HealthStatus::Ok
        } else {
            HealthStatus::Degraded(degraded.join("; "))
        }
    }

    pub fn components(&self) -> Vec<&ComponentHealth> {
        self.components.values().collect()
    }

    pub fn get(&self, name: &str) -> Option<&ComponentHealth> {
        self.components.get(name)
    }
}

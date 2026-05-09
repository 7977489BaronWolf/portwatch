//! Context enrichment for port events — attaches runtime metadata
//! (hostname, process name, user) to detected port changes.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PortContext {
    pub port: u16,
    pub protocol: String,
    pub hostname: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub username: Option<String>,
    pub extra: HashMap<String, String>,
}

impl PortContext {
    pub fn new(port: u16, protocol: impl Into<String>) -> Self {
        Self {
            port,
            protocol: protocol.into(),
            hostname: hostname(),
            pid: None,
            process_name: None,
            username: None,
            extra: HashMap::new(),
        }
    }

    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    pub fn with_process(mut self, name: impl Into<String>) -> Self {
        self.process_name = Some(name.into());
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.username = Some(user.into());
        self
    }

    pub fn insert_extra(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.extra.insert(key.into(), value.into());
    }

    pub fn summary(&self) -> String {
        let proc = self.process_name.as_deref().unwrap_or("unknown");
        let user = self.username.as_deref().unwrap_or("unknown");
        format!(
            "{}:{} on {} (process={}, user={})",
            self.protocol, self.port, self.hostname, proc, user
        )
    }
}

fn hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
}

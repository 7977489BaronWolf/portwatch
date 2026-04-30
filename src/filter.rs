//! Port filter module — allows users to define rules for ignoring certain ports or protocols.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRule {
    pub port: Option<u16>,
    pub port_range: Option<(u16, u16)>,
    pub protocol: Protocol,
    pub comment: Option<String>,
}

impl FilterRule {
    pub fn matches(&self, port: u16, proto: &Protocol) -> bool {
        let proto_match = matches!(
            (&self.protocol, proto),
            (Protocol::Any, _)
                | (Protocol::Tcp, Protocol::Tcp)
                | (Protocol::Udp, Protocol::Udp)
        );
        if !proto_match {
            return false;
        }
        if let Some(p) = self.port {
            return p == port;
        }
        if let Some((lo, hi)) = self.port_range {
            return port >= lo && port <= hi;
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterSet {
    pub rules: Vec<FilterRule>,
}

impl FilterSet {
    pub fn new(rules: Vec<FilterRule>) -> Self {
        Self { rules }
    }

    /// Returns `true` if the port/protocol pair should be ignored.
    pub fn is_ignored(&self, port: u16, proto: &Protocol) -> bool {
        self.rules.iter().any(|r| r.matches(port, proto))
    }

    /// Filter a list of (port, protocol) tuples, removing ignored entries.
    pub fn apply(&self, ports: &[(u16, Protocol)]) -> Vec<(u16, Protocol)> {
        ports
            .iter()
            .filter(|(p, proto)| !self.is_ignored(*p, proto))
            .cloned()
            .collect()
    }
}

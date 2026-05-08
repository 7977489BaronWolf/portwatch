//! Port event enrichment: attaches metadata (service name, protocol, risk hints)
//! to detected port changes before they are dispatched as alerts.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct EnrichedPort {
    pub port: u16,
    pub protocol: String,
    pub service_name: Option<String>,
    pub risk_hint: RiskHint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskHint {
    Low,
    Medium,
    High,
}

pub struct Enricher {
    service_map: HashMap<u16, (&'static str, RiskHint)>,
}

impl Enricher {
    pub fn new() -> Self {
        let mut service_map: HashMap<u16, (&'static str, RiskHint)> = HashMap::new();
        service_map.insert(22,   ("ssh",   RiskHint::Medium));
        service_map.insert(23,   ("telnet", RiskHint::High));
        service_map.insert(25,   ("smtp",  RiskHint::Medium));
        service_map.insert(53,   ("dns",   RiskHint::Low));
        service_map.insert(80,   ("http",  RiskHint::Low));
        service_map.insert(443,  ("https", RiskHint::Low));
        service_map.insert(3306, ("mysql", RiskHint::High));
        service_map.insert(5432, ("postgres", RiskHint::High));
        service_map.insert(6379, ("redis", RiskHint::High));
        service_map.insert(27017,("mongodb", RiskHint::High));
        Enricher { service_map }
    }

    pub fn enrich(&self, port: u16, protocol: &str) -> EnrichedPort {
        let (service_name, risk_hint) = match self.service_map.get(&port) {
            Some((name, hint)) => (Some(name.to_string()), hint.clone()),
            None => {
                let hint = if port < 1024 {
                    RiskHint::Medium
                } else {
                    RiskHint::Low
                };
                (None, hint)
            }
        };
        EnrichedPort {
            port,
            protocol: protocol.to_string(),
            service_name,
            risk_hint,
        }
    }

    pub fn enrich_many(&self, ports: &[(u16, &str)]) -> Vec<EnrichedPort> {
        ports.iter().map(|(p, proto)| self.enrich(*p, proto)).collect()
    }
}

impl Default for Enricher {
    fn default() -> Self {
        Self::new()
    }
}

//! Port fingerprinting module — identifies service signatures on open ports.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Fingerprint {
    pub port: u16,
    pub protocol: String,
    pub service: Option<String>,
    pub banner: Option<String>,
    pub confidence: u8, // 0–100
}

impl Fingerprint {
    pub fn new(port: u16, protocol: impl Into<String>) -> Self {
        Self {
            port,
            protocol: protocol.into(),
            service: None,
            banner: None,
            confidence: 0,
        }
    }

    pub fn with_service(mut self, service: impl Into<String>, confidence: u8) -> Self {
        self.service = Some(service.into());
        self.confidence = confidence.min(100);
        self
    }

    pub fn with_banner(mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self
    }

    pub fn is_known(&self) -> bool {
        self.service.is_some() && self.confidence >= 50
    }
}

/// Matches a port/protocol pair against a built-in well-known service table.
pub fn identify(port: u16, protocol: &str) -> Fingerprint {
    let mut fp = Fingerprint::new(port, protocol);
    if let Some((service, confidence)) = WELL_KNOWN.get(&(port, protocol)) {
        fp = fp.with_service(*service, *confidence);
    }
    fp
}

/// Attempt to enrich a fingerprint with a raw banner string.
pub fn enrich_with_banner(fp: Fingerprint, banner: &str) -> Fingerprint {
    let trimmed = banner.trim().to_string();
    if trimmed.is_empty() {
        return fp;
    }
    let mut enriched = fp.with_banner(trimmed.clone());
    // Boost confidence if banner contains the expected service name
    if let Some(ref svc) = enriched.service.clone() {
        if trimmed.to_lowercase().contains(&svc.to_lowercase()) {
            enriched.confidence = (enriched.confidence + 15).min(100);
        }
    }
    enriched
}

static WELL_KNOWN: std::sync::LazyLock<HashMap<(u16, &'static str), (&'static str, u8)>> =
    std::sync::LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert((22, "tcp"), ("ssh", 90));
        m.insert((80, "tcp"), ("http", 90));
        m.insert((443, "tcp"), ("https", 90));
        m.insert((21, "tcp"), ("ftp", 85));
        m.insert((25, "tcp"), ("smtp", 85));
        m.insert((53, "tcp"), ("dns", 80));
        m.insert((53, "udp"), ("dns", 80));
        m.insert((3306, "tcp"), ("mysql", 85));
        m.insert((5432, "tcp"), ("postgresql", 85));
        m.insert((6379, "tcp"), ("redis", 85));
        m.insert((27017, "tcp"), ("mongodb", 85));
        m.insert((8080, "tcp"), ("http-alt", 75));
        m.insert((8443, "tcp"), ("https-alt", 75));
        m
    });

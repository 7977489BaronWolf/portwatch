use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

/// Represents an open port with its protocol and associated process info.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortEntry {
    pub protocol: String,
    pub local_port: u16,
    pub state: String,
    pub pid: Option<u32>,
}

impl PortEntry {
    pub fn new(protocol: &str, local_port: u16, state: &str, pid: Option<u32>) -> Self {
        Self {
            protocol: protocol.to_string(),
            local_port,
            state: state.to_string(),
            pid,
        }
    }
}

/// Scans /proc/net/tcp and /proc/net/tcp6 for listening ports.
/// Returns a set of PortEntry structs representing currently open ports.
pub fn scan_open_ports() -> io::Result<HashSet<PortEntry>> {
    let mut entries = HashSet::new();

    for (path, proto) in &[
        ("/proc/net/tcp", "tcp"),
        ("/proc/net/tcp6", "tcp6"),
        ("/proc/net/udp", "udp"),
        ("/proc/net/udp6", "udp6"),
    ] {
        if let Ok(file) = fs::File::open(path) {
            let reader = io::BufReader::new(file);
            for line in reader.lines().skip(1) {
                let line = line?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    continue;
                }
                // state 0A = LISTEN for TCP, 07 = CLOSE for UDP (active)
                let state_hex = parts[3];
                let is_relevant = state_hex == "0A" || state_hex == "07";
                if !is_relevant {
                    continue;
                }
                if let Some(port) = parse_port_from_hex(parts[1]) {
                    let state = if state_hex == "0A" { "LISTEN" } else { "ACTIVE" };
                    entries.insert(PortEntry::new(proto, port, state, None));
                }
            }
        }
    }

    Ok(entries)
}

/// Parses a hex-encoded local address field (e.g., "0F02000A:1F90") into a port number.
fn parse_port_from_hex(addr_field: &str) -> Option<u16> {
    let port_hex = addr_field.split(':').nth(1)?;
    u16::from_str_radix(port_hex, 16).ok()
}

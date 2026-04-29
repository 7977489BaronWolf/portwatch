#[cfg(test)]
mod tests {
    use super::super::port_scanner::{parse_port_from_hex, PortEntry, scan_open_ports};

    // Re-expose private fn for testing via a thin wrapper in the module
    // We test the public surface and data structures here.

    #[test]
    fn port_entry_equality() {
        let a = PortEntry::new("tcp", 8080, "LISTEN", None);
        let b = PortEntry::new("tcp", 8080, "LISTEN", None);
        assert_eq!(a, b);
    }

    #[test]
    fn port_entry_inequality_on_port() {
        let a = PortEntry::new("tcp", 8080, "LISTEN", None);
        let b = PortEntry::new("tcp", 9090, "LISTEN", None);
        assert_ne!(a, b);
    }

    #[test]
    fn port_entry_inequality_on_protocol() {
        let a = PortEntry::new("tcp", 8080, "LISTEN", None);
        let b = PortEntry::new("udp", 8080, "ACTIVE", None);
        assert_ne!(a, b);
    }

    #[test]
    fn port_entry_in_hashset() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(PortEntry::new("tcp", 80, "LISTEN", None));
        set.insert(PortEntry::new("tcp", 80, "LISTEN", None)); // duplicate
        set.insert(PortEntry::new("tcp", 443, "LISTEN", None));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn scan_open_ports_returns_result() {
        // On Linux with /proc available this should succeed;
        // on other platforms it gracefully returns an empty set.
        let result = scan_open_ports();
        assert!(result.is_ok(), "scan_open_ports should not error");
    }

    #[test]
    fn scan_ports_are_valid_port_numbers() {
        if let Ok(ports) = scan_open_ports() {
            for entry in &ports {
                assert!(entry.local_port > 0, "port must be > 0");
                assert!(
                    entry.state == "LISTEN" || entry.state == "ACTIVE",
                    "unexpected state: {}",
                    entry.state
                );
            }
        }
    }
}

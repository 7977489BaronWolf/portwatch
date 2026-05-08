#[cfg(test)]
mod tests {
    use crate::enrichment::{Enricher, RiskHint};

    fn enricher() -> Enricher {
        Enricher::new()
    }

    #[test]
    fn test_known_port_ssh() {
        let e = enricher();
        let result = e.enrich(22, "tcp");
        assert_eq!(result.port, 22);
        assert_eq!(result.service_name, Some("ssh".to_string()));
        assert_eq!(result.risk_hint, RiskHint::Medium);
        assert_eq!(result.protocol, "tcp");
    }

    #[test]
    fn test_known_port_telnet_high_risk() {
        let e = enricher();
        let result = e.enrich(23, "tcp");
        assert_eq!(result.service_name, Some("telnet".to_string()));
        assert_eq!(result.risk_hint, RiskHint::High);
    }

    #[test]
    fn test_known_port_https_low_risk() {
        let e = enricher();
        let result = e.enrich(443, "tcp");
        assert_eq!(result.service_name, Some("https".to_string()));
        assert_eq!(result.risk_hint, RiskHint::Low);
    }

    #[test]
    fn test_unknown_privileged_port_medium_risk() {
        let e = enricher();
        let result = e.enrich(999, "tcp");
        assert_eq!(result.service_name, None);
        assert_eq!(result.risk_hint, RiskHint::Medium);
    }

    #[test]
    fn test_unknown_ephemeral_port_low_risk() {
        let e = enricher();
        let result = e.enrich(54321, "udp");
        assert_eq!(result.service_name, None);
        assert_eq!(result.risk_hint, RiskHint::Low);
        assert_eq!(result.protocol, "udp");
    }

    #[test]
    fn test_database_ports_high_risk() {
        let e = enricher();
        for port in [3306u16, 5432, 6379, 27017] {
            let result = e.enrich(port, "tcp");
            assert_eq!(result.risk_hint, RiskHint::High,
                "port {} should be high risk", port);
        }
    }

    #[test]
    fn test_enrich_many() {
        let e = enricher();
        let ports = vec![(80u16, "tcp"), (22u16, "tcp"), (9999u16, "tcp")];
        let results = e.enrich_many(&ports);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].service_name, Some("http".to_string()));
        assert_eq!(results[1].service_name, Some("ssh".to_string()));
        assert_eq!(results[2].service_name, None);
    }

    #[test]
    fn test_enrich_many_empty() {
        let e = enricher();
        let results = e.enrich_many(&[]);
        assert!(results.is_empty());
    }
}

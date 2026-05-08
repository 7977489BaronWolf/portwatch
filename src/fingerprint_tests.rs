#[cfg(test)]
mod tests {
    use crate::fingerprint::*;

    #[test]
    fn test_new_fingerprint_defaults() {
        let fp = Fingerprint::new(8080, "tcp");
        assert_eq!(fp.port, 8080);
        assert_eq!(fp.protocol, "tcp");
        assert!(fp.service.is_none());
        assert!(fp.banner.is_none());
        assert_eq!(fp.confidence, 0);
    }

    #[test]
    fn test_with_service_clamps_confidence() {
        let fp = Fingerprint::new(22, "tcp").with_service("ssh", 200);
        assert_eq!(fp.confidence, 100);
        assert_eq!(fp.service.as_deref(), Some("ssh"));
    }

    #[test]
    fn test_is_known_requires_service_and_confidence() {
        let low = Fingerprint::new(22, "tcp").with_service("ssh", 40);
        assert!(!low.is_known());

        let high = Fingerprint::new(22, "tcp").with_service("ssh", 90);
        assert!(high.is_known());
    }

    #[test]
    fn test_identify_known_port() {
        let fp = identify(22, "tcp");
        assert_eq!(fp.service.as_deref(), Some("ssh"));
        assert!(fp.confidence >= 50);
        assert!(fp.is_known());
    }

    #[test]
    fn test_identify_unknown_port() {
        let fp = identify(9999, "tcp");
        assert!(fp.service.is_none());
        assert!(!fp.is_known());
    }

    #[test]
    fn test_identify_udp_dns() {
        let fp = identify(53, "udp");
        assert_eq!(fp.service.as_deref(), Some("dns"));
    }

    #[test]
    fn test_enrich_with_banner_sets_banner() {
        let fp = identify(22, "tcp");
        let enriched = enrich_with_banner(fp, "SSH-2.0-OpenSSH_9.0");
        assert!(enriched.banner.is_some());
        assert!(enriched.banner.as_deref().unwrap().contains("OpenSSH"));
    }

    #[test]
    fn test_enrich_banner_boosts_confidence_on_match() {
        let fp = identify(6379, "tcp"); // redis, confidence 85
        let before = fp.confidence;
        let enriched = enrich_with_banner(fp, "redis 7.0.5 server ready");
        assert!(enriched.confidence > before || enriched.confidence == 100);
    }

    #[test]
    fn test_enrich_empty_banner_no_change() {
        let fp = identify(80, "tcp");
        let cloned = fp.clone();
        let enriched = enrich_with_banner(fp, "   ");
        assert_eq!(enriched.service, cloned.service);
        assert_eq!(enriched.confidence, cloned.confidence);
        assert!(enriched.banner.is_none());
    }

    #[test]
    fn test_fingerprint_serialization() {
        let fp = identify(443, "tcp");
        let json = serde_json::to_string(&fp).expect("serialize");
        let back: Fingerprint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(fp, back);
    }
}

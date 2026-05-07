#[cfg(test)]
mod tests {
    use super::*;
    use crate::correlation::{CorrelationEngine, CorrelationGroup};
    use crate::diff_engine::{PortDiff, DiffKind};
    use std::time::Duration;

    fn make_diff(port: u16) -> PortDiff {
        PortDiff {
            port,
            kind: DiffKind::Opened,
            protocol: "tcp".to_string(),
            process: None,
        }
    }

    #[test]
    fn test_empty_flush_returns_no_incidents() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        let incidents = engine.flush();
        assert!(incidents.is_empty());
    }

    #[test]
    fn test_single_diff_unknown_group() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        engine.ingest(make_diff(8080));
        let incidents = engine.flush();
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].group, CorrelationGroup::Unknown);
        assert_eq!(incidents[0].diffs.len(), 1);
    }

    #[test]
    fn test_mass_scan_detected() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        for port in 1000..1010 {
            engine.ingest(make_diff(port));
        }
        let incidents = engine.flush();
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].group, CorrelationGroup::MassScan);
    }

    #[test]
    fn test_port_flap_detected() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        for _ in 0..3 {
            engine.ingest(make_diff(443));
        }
        let incidents = engine.flush();
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].group, CorrelationGroup::PortFlap);
    }

    #[test]
    fn test_new_service_group() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        engine.ingest(make_diff(3000));
        engine.ingest(make_diff(3001));
        engine.ingest(make_diff(3002));
        let incidents = engine.flush();
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].group, CorrelationGroup::NewService);
    }

    #[test]
    fn test_flush_clears_pending() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        engine.ingest(make_diff(22));
        let _ = engine.flush();
        let second = engine.flush();
        assert!(second.is_empty());
    }

    #[test]
    fn test_incident_has_summary() {
        let mut engine = CorrelationEngine::new(Duration::from_secs(60));
        engine.ingest(make_diff(80));
        let incidents = engine.flush();
        assert!(!incidents[0].summary.is_empty());
        assert!(incidents[0].summary.contains("port change"));
    }
}

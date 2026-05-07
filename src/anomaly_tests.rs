#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::anomaly::{AnomalyDetector, AnomalyKind};

    fn make_detector(threshold: u32) -> AnomalyDetector {
        AnomalyDetector::new(threshold, HashMap::new())
    }

    #[test]
    fn no_anomaly_below_threshold() {
        let mut d = make_detector(5);
        let result = d.record_change(8080, "tcp");
        assert!(result.is_none());
    }

    #[test]
    fn anomaly_at_threshold() {
        let mut d = make_detector(3);
        d.record_change(9000, "tcp");
        d.record_change(9000, "tcp");
        let result = d.record_change(9000, "tcp");
        assert!(result.is_some());
        let anomaly = result.unwrap();
        assert_eq!(anomaly.port, 9000);
        assert_eq!(anomaly.kind, AnomalyKind::FrequentOpen);
        assert!(anomaly.score >= 1.0);
    }

    #[test]
    fn score_increases_beyond_threshold() {
        let mut d = make_detector(2);
        d.record_change(443, "tcp");
        d.record_change(443, "tcp");
        let result = d.record_change(443, "tcp");
        let anomaly = result.unwrap();
        assert!(anomaly.score > 1.0);
    }

    #[test]
    fn protocol_mismatch_detected() {
        let mut protocols = HashMap::new();
        protocols.insert(80u16, "tcp".to_string());
        let mut d = AnomalyDetector::new(10, protocols);
        let result = d.record_change(80, "udp");
        assert!(result.is_some());
        let anomaly = result.unwrap();
        assert_eq!(anomaly.kind, AnomalyKind::UnexpectedProtocol);
        assert!(anomaly.description.contains("udp"));
    }

    #[test]
    fn protocol_match_no_anomaly() {
        let mut protocols = HashMap::new();
        protocols.insert(443u16, "tcp".to_string());
        let mut d = AnomalyDetector::new(10, protocols);
        let result = d.record_change(443, "tcp");
        assert!(result.is_none());
    }

    #[test]
    fn reset_clears_counts() {
        let mut d = make_detector(3);
        d.record_change(22, "tcp");
        d.record_change(22, "tcp");
        assert_eq!(d.change_count(22), 2);
        d.reset_counts();
        assert_eq!(d.change_count(22), 0);
    }

    #[test]
    fn independent_ports_tracked_separately() {
        let mut d = make_detector(5);
        d.record_change(80, "tcp");
        d.record_change(80, "tcp");
        d.record_change(443, "tcp");
        assert_eq!(d.change_count(80), 2);
        assert_eq!(d.change_count(443), 1);
    }

    #[test]
    fn unknown_port_count_returns_zero() {
        let d = make_detector(5);
        assert_eq!(d.change_count(9999), 0);
    }
}

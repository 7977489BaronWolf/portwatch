#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::metrics::{Metrics, timed_scan};

    #[test]
    fn test_initial_snapshot_is_zero() {
        let m = Metrics::new();
        let s = m.snapshot();
        assert_eq!(s.scans_total, 0);
        assert_eq!(s.alerts_total, 0);
        assert_eq!(s.errors_total, 0);
        assert_eq!(s.last_scan_duration_ms, 0);
        assert_eq!(s.ports_open_last, 0);
    }

    #[test]
    fn test_record_scan_increments_counter() {
        let m = Metrics::new();
        m.record_scan(Duration::from_millis(42), 5);
        let s = m.snapshot();
        assert_eq!(s.scans_total, 1);
        assert_eq!(s.last_scan_duration_ms, 42);
        assert_eq!(s.ports_open_last, 5);
    }

    #[test]
    fn test_record_scan_multiple_times() {
        let m = Metrics::new();
        m.record_scan(Duration::from_millis(10), 3);
        m.record_scan(Duration::from_millis(20), 7);
        let s = m.snapshot();
        assert_eq!(s.scans_total, 2);
        assert_eq!(s.last_scan_duration_ms, 20);
        assert_eq!(s.ports_open_last, 7);
    }

    #[test]
    fn test_record_alert_increments_counter() {
        let m = Metrics::new();
        m.record_alert();
        m.record_alert();
        assert_eq!(m.snapshot().alerts_total, 2);
    }

    #[test]
    fn test_record_error_increments_counter() {
        let m = Metrics::new();
        m.record_error();
        assert_eq!(m.snapshot().errors_total, 1);
    }

    #[test]
    fn test_timed_scan_returns_open_ports() {
        let m = Metrics::new();
        let result = timed_scan(&m, || 9);
        assert_eq!(result, 9);
        let s = m.snapshot();
        assert_eq!(s.scans_total, 1);
        assert_eq!(s.ports_open_last, 9);
    }

    #[test]
    fn test_display_format() {
        let m = Metrics::new();
        m.record_scan(Duration::from_millis(55), 4);
        m.record_alert();
        m.record_error();
        let display = format!("{}", m.snapshot());
        assert!(display.contains("scans=1"));
        assert!(display.contains("alerts=1"));
        assert!(display.contains("errors=1"));
        assert!(display.contains("last_scan_ms=55"));
        assert!(display.contains("open_ports=4"));
    }
}

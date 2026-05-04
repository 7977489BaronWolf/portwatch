#[cfg(test)]
mod tests {
    use super::super::audit_integration::*;
    use super::super::audit_log::{AuditLog, AuditEventType};
    use super::super::diff_engine::PortDiff;
    use tempfile::tempdir;

    #[test]
    fn test_record_opened_port_diff() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        let diffs = vec![
            PortDiff::Opened { port: 8080, protocol: "tcp".to_string() },
        ];
        record_diff_events(&log, &diffs).unwrap();

        let entries = log.read_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].port, Some(8080));
        assert!(entries[0].description.contains("opened"));
    }

    #[test]
    fn test_record_closed_port_diff() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        let diffs = vec![
            PortDiff::Closed { port: 22, protocol: "tcp".to_string() },
        ];
        record_diff_events(&log, &diffs).unwrap();

        let entries = log.read_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].description.contains("closed"));
        assert_eq!(entries[0].port, Some(22));
    }

    #[test]
    fn test_record_multiple_diffs() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        let diffs = vec![
            PortDiff::Opened { port: 443, protocol: "tcp".to_string() },
            PortDiff::Closed { port: 80, protocol: "tcp".to_string() },
            PortDiff::Opened { port: 9000, protocol: "udp".to_string() },
        ];
        record_diff_events(&log, &diffs).unwrap();

        let entries = log.read_entries().unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_open_audit_log_creates_file() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("sub").join("audit.log");
        let log_path_str = log_path.to_str().unwrap().to_string();

        let log = open_audit_log(&log_path_str).unwrap();
        log.record(AuditEventType::DaemonStarted, "started", None, None).unwrap();

        assert!(log.path().exists());
    }
}

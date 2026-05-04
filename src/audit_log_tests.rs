#[cfg(test)]
mod tests {
    use super::super::audit_log::*;
    use tempfile::tempdir;

    #[test]
    fn test_record_and_read_single_entry() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        log.record(AuditEventType::PortOpened, "Port 8080 opened", Some(8080), Some("tcp")).unwrap();

        let entries = log.read_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].port, Some(8080));
        assert_eq!(entries[0].description, "Port 8080 opened");
    }

    #[test]
    fn test_record_multiple_entries() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        log.record(AuditEventType::DaemonStarted, "Daemon started", None, None).unwrap();
        log.record(AuditEventType::PortOpened, "Port 443 opened", Some(443), Some("tcp")).unwrap();
        log.record(AuditEventType::AlertFired, "Alert sent for port 443", Some(443), None).unwrap();

        let entries = log.read_entries().unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_read_entries_empty_file() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        let entries = log.read_entries().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_entry_has_timestamp() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        log.record(AuditEventType::PortClosed, "Port 22 closed", Some(22), Some("tcp")).unwrap();

        let entries = log.read_entries().unwrap();
        assert!(!entries[0].timestamp.is_empty());
    }

    #[test]
    fn test_creates_parent_directories() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("nested").join("dir").join("audit.log");
        let log = AuditLog::new(&log_path).unwrap();

        log.record(AuditEventType::BaselineUpdated, "Baseline updated", None, None).unwrap();

        assert!(log.path().exists());
    }
}

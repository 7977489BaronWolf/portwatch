use crate::audit_log::{AuditLog, AuditEventType};
use crate::diff_engine::PortDiff;
use std::path::Path;

/// Records port change events from a diff result into the audit log.
pub fn record_diff_events(
    log: &AuditLog,
    diffs: &[PortDiff],
) -> std::io::Result<()> {
    for diff in diffs {
        match diff {
            PortDiff::Opened { port, protocol } => {
                let desc = format!("Port {}/{} opened", port, protocol);
                log.record(
                    AuditEventType::PortOpened,
                    &desc,
                    Some(*port),
                    Some(protocol.as_str()),
                )?;
            }
            PortDiff::Closed { port, protocol } => {
                let desc = format!("Port {}/{} closed", port, protocol);
                log.record(
                    AuditEventType::PortClosed,
                    &desc,
                    Some(*port),
                    Some(protocol.as_str()),
                )?;
            }
        }
    }
    Ok(())
}

/// Convenience function to get or create an audit log from config path.
pub fn open_audit_log(log_path: &str) -> std::io::Result<AuditLog> {
    AuditLog::new(Path::new(log_path))
}

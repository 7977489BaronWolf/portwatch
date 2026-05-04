use std::path::Path;
use crate::audit_log::{AuditLog, AuditEventType};

pub struct AuditCmdOptions {
    pub log_path: String,
    pub filter_event: Option<String>,
    pub limit: Option<usize>,
}

pub fn run_audit_cmd(opts: &AuditCmdOptions) -> Result<(), Box<dyn std::error::Error>> {
    let log = AuditLog::new(Path::new(&opts.log_path))?;
    let entries = log.read_entries()?;

    if entries.is_empty() {
        println!("No audit log entries found.");
        return Ok(());
    }

    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| {
            if let Some(ref filter) = opts.filter_event {
                let type_str = format!("{:?}", e.event_type).to_lowercase();
                type_str.contains(&filter.to_lowercase())
            } else {
                true
            }
        })
        .collect();

    let limited: Vec<_> = match opts.limit {
        Some(n) => filtered.into_iter().rev().take(n).collect(),
        None => filtered.into_iter().rev().collect(),
    };

    println!("{:<30} {:<20} {:<10} {}",
        "Timestamp", "Event", "Port", "Description");
    println!("{}", "-".repeat(80));

    for entry in limited.iter().rev() {
        let port_str = entry.port
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());
        let event_str = format!("{:?}", entry.event_type);
        println!("{:<30} {:<20} {:<10} {}",
            &entry.timestamp[..19],
            event_str,
            port_str,
            entry.description
        );
    }

    Ok(())
}

pub fn record_daemon_event(
    log_path: &str,
    event_type: AuditEventType,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let log = AuditLog::new(Path::new(log_path))?;
    log.record(event_type, description, None, None)?;
    Ok(())
}

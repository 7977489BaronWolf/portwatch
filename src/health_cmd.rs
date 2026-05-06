//! CLI command handler for `portwatch health` subcommand.
//!
//! Prints the health status of all registered daemon components.

use crate::health::{HealthRegistry, HealthStatus};
use std::io::Write;

pub struct HealthCmdOptions {
    /// Show only failed/degraded components
    pub failures_only: bool,
    /// Output as JSON
    pub json: bool,
}

pub fn run_health_cmd<W: Write>(
    registry: &HealthRegistry,
    opts: &HealthCmdOptions,
    out: &mut W,
) -> anyhow::Result<()> {
    let overall = registry.overall_status();
    let overall_label = status_label(&overall);

    if opts.json {
        writeln!(out, "{{")?;
        writeln!(out, "  \"overall\": \"{}\",", overall_label)?;
        writeln!(out, "  \"components\": [")?;
        let components: Vec<_> = registry.components().into_iter().collect();
        for (i, c) in components.iter().enumerate() {
            let label = status_label(&c.status);
            let detail = status_detail(&c.status);
            let comma = if i + 1 < components.len() { "," } else { "" };
            writeln!(
                out,
                "    {{\"name\": \"{}\", \"status\": \"{}\", \"detail\": \"{}\"}}{}",
                c.name, label, detail, comma
            )?;
        }
        writeln!(out, "  ]")?;
        writeln!(out, "}}")?
    } else {
        writeln!(out, "Overall: {}", overall_label)?;
        writeln!(out, "---")?;
        for c in registry.components() {
            if opts.failures_only && c.is_healthy() {
                continue;
            }
            let label = status_label(&c.status);
            let detail = status_detail(&c.status);
            if detail.is_empty() {
                writeln!(out, "  [{label}] {}", c.name)?;
            } else {
                writeln!(out, "  [{label}] {}: {}", c.name, detail)?;
            }
        }
    }

    Ok(())
}

fn status_label(s: &HealthStatus) -> &'static str {
    match s {
        HealthStatus::Ok => "OK",
        HealthStatus::Degraded(_) => "DEGRADED",
        HealthStatus::Failed(_) => "FAILED",
    }
}

fn status_detail(s: &HealthStatus) -> String {
    match s {
        HealthStatus::Ok => String::new(),
        HealthStatus::Degraded(m) | HealthStatus::Failed(m) => m.clone(),
    }
}

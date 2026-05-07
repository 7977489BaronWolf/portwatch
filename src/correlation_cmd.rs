//! CLI sub-command handler for correlation incident reporting.

use crate::correlation::{CorrelationEngine, CorrelationGroup};
use crate::diff_engine::{PortDiff, DiffKind};
use std::time::Duration;

#[derive(Debug)]
pub struct CorrelationCmdOptions {
    pub window_secs: u64,
    pub json_output: bool,
}

impl Default for CorrelationCmdOptions {
    fn default() -> Self {
        Self {
            window_secs: 60,
            json_output: false,
        }
    }
}

pub fn run_correlation_cmd(opts: CorrelationCmdOptions, diffs: Vec<PortDiff>) {
    let mut engine = CorrelationEngine::new(Duration::from_secs(opts.window_secs));
    for diff in diffs {
        engine.ingest(diff);
    }
    let incidents = engine.flush();

    if incidents.is_empty() {
        println!("No incidents detected in the current window.");
        return;
    }

    for inc in &incidents {
        if opts.json_output {
            println!(
                "{{\"id\":\"{}\",\"group\":\"{:?}\",\"changes\":{},\"summary":\"{}\"}}" ,
                inc.id,
                inc.group,
                inc.diffs.len(),
                inc.summary
            );
        } else {
            println!("[{}] {:?} — {}", inc.id, inc.group, inc.summary);
            for d in &inc.diffs {
                println!("  port={} kind={:?} proto={}", d.port, d.kind, d.protocol);
            }
        }
    }
}

pub fn group_label(group: &CorrelationGroup) -> &'static str {
    match group {
        CorrelationGroup::MassScan => "Mass Scan",
        CorrelationGroup::ServiceRestart => "Service Restart",
        CorrelationGroup::NewService => "New Service",
        CorrelationGroup::PortFlap => "Port Flap",
        CorrelationGroup::Unknown => "Unknown",
    }
}

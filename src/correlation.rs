//! Alert correlation engine: groups related port change events into incidents.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use crate::diff_engine::PortDiff;

#[derive(Debug, Clone, PartialEq)]
pub enum CorrelationGroup {
    MassScan,
    ServiceRestart,
    NewService,
    PortFlap,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Incident {
    pub id: String,
    pub group: CorrelationGroup,
    pub diffs: Vec<PortDiff>,
    pub detected_at: SystemTime,
    pub summary: String,
}

pub struct CorrelationEngine {
    window: Duration,
    pending: Vec<(SystemTime, PortDiff)>,
}

impl CorrelationEngine {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            pending: Vec::new(),
        }
    }

    pub fn ingest(&mut self, diff: PortDiff) {
        self.pending.push((SystemTime::now(), diff));
    }

    pub fn flush(&mut self) -> Vec<Incident> {
        let cutoff = SystemTime::now() - self.window;
        self.pending.retain(|(t, _)| *t >= cutoff);

        if self.pending.is_empty() {
            return vec![];
        }

        let mut port_counts: HashMap<u16, usize> = HashMap::new();
        for (_, d) in &self.pending {
            *port_counts.entry(d.port).or_insert(0) += 1;
        }

        let total = self.pending.len();
        let group = if total >= 10 {
            CorrelationGroup::MassScan
        } else if port_counts.values().any(|&c| c >= 3) {
            CorrelationGroup::PortFlap
        } else if total >= 3 {
            CorrelationGroup::NewService
        } else {
            CorrelationGroup::Unknown
        };

        let diffs: Vec<PortDiff> = self.pending.iter().map(|(_, d)| d.clone()).collect();
        let summary = format!("{} port change(s) grouped as {:?}", total, group);
        let incident = Incident {
            id: uuid_simple(),
            group,
            diffs,
            detected_at: SystemTime::now(),
            summary,
        };

        self.pending.clear();
        vec![incident]
    }
}

fn uuid_simple() -> String {
    use std::time::UNIX_EPOCH;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("inc-{:08x}", nanos)
}

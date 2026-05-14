//! Pipeline module — chains processing stages for port change events.

use crate::diff_engine::PortDiff;
use crate::filter::Filter;
use crate::dedup::Dedup;
use crate::enrichment::Enrichment;
use crate::severity::Severity;

#[derive(Debug, Clone)]
pub struct PipelineEvent {
    pub diff: PortDiff,
    pub severity: Severity,
    pub tags: Vec<String>,
    pub suppressed: bool,
}

impl PipelineEvent {
    pub fn new(diff: PortDiff) -> Self {
        Self {
            diff,
            severity: Severity::Info,
            tags: Vec::new(),
            suppressed: false,
        }
    }
}

pub struct Pipeline {
    filter: Filter,
    dedup: Dedup,
    enrichment: Enrichment,
}

impl Pipeline {
    pub fn new(filter: Filter, dedup: Dedup, enrichment: Enrichment) -> Self {
        Self { filter, dedup, enrichment }
    }

    /// Run a batch of diffs through all pipeline stages.
    /// Returns only the events that survive filtering and deduplication.
    pub fn process(&mut self, diffs: Vec<PortDiff>) -> Vec<PipelineEvent> {
        let mut events: Vec<PipelineEvent> = diffs
            .into_iter()
            .filter(|d| self.filter.matches(d))
            .filter(|d| !self.dedup.is_duplicate(d))
            .map(PipelineEvent::new)
            .collect();

        for event in &mut events {
            self.enrichment.enrich(event);
        }

        events.retain(|e| !e.suppressed);
        events
    }

    pub fn reset(&mut self) {
        self.dedup.clear();
    }
}

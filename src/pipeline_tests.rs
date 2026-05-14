#[cfg(test)]
mod tests {
    use crate::pipeline::{Pipeline, PipelineEvent};
    use crate::diff_engine::PortDiff;
    use crate::filter::Filter;
    use crate::dedup::Dedup;
    use crate::enrichment::Enrichment;
    use crate::severity::Severity;

    fn make_diff(port: u16, kind: &str) -> PortDiff {
        PortDiff { port, protocol: "tcp".into(), kind: kind.into() }
    }

    fn default_pipeline() -> Pipeline {
        Pipeline::new(
            Filter::allow_all(),
            Dedup::new(),
            Enrichment::default(),
        )
    }

    #[test]
    fn test_empty_input_returns_empty() {
        let mut pipeline = default_pipeline();
        let result = pipeline.process(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_diff_passes_through() {
        let mut pipeline = default_pipeline();
        let diffs = vec![make_diff(8080, "opened")];
        let events = pipeline.process(diffs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].diff.port, 8080);
    }

    #[test]
    fn test_duplicate_is_removed() {
        let mut pipeline = default_pipeline();
        let diff = make_diff(443, "opened");
        // First pass — should pass through
        let first = pipeline.process(vec![diff.clone()]);
        assert_eq!(first.len(), 1);
        // Second pass — duplicate, should be filtered
        let second = pipeline.process(vec![diff]);
        assert!(second.is_empty());
    }

    #[test]
    fn test_reset_clears_dedup_state() {
        let mut pipeline = default_pipeline();
        let diff = make_diff(22, "opened");
        pipeline.process(vec![diff.clone()]);
        pipeline.reset();
        let after_reset = pipeline.process(vec![diff]);
        assert_eq!(after_reset.len(), 1);
    }

    #[test]
    fn test_filter_blocks_unwanted_diffs() {
        let mut pipeline = Pipeline::new(
            Filter::block_all(),
            Dedup::new(),
            Enrichment::default(),
        );
        let diffs = vec![make_diff(80, "opened"), make_diff(443, "opened")];
        let events = pipeline.process(diffs);
        assert!(events.is_empty());
    }

    #[test]
    fn test_enrichment_sets_severity() {
        let mut pipeline = default_pipeline();
        let events = pipeline.process(vec![make_diff(22, "opened")]);
        assert_eq!(events.len(), 1);
        // Enrichment should assign a non-default severity for well-known ports
        assert_ne!(events[0].severity, Severity::Unknown);
    }
}

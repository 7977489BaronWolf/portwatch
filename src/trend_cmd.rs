//! CLI subcommand handler for trend reporting.

use crate::trend::{TrendDirection, TrendTracker};
use crate::trend_store::TrendStore;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrendCmdOptions {
    pub store_path: String,
    pub window_secs: u64,
    pub max_samples: usize,
}

impl Default for TrendCmdOptions {
    fn default() -> Self {
        Self {
            store_path: "/var/lib/portwatch/trend.json".to_string(),
            window_secs: 3600,
            max_samples: 100,
        }
    }
}

pub fn run_trend_report(opts: &TrendCmdOptions) -> String {
    let store = TrendStore::new(&opts.store_path, opts.window_secs * 2);
    let mut tracker = TrendTracker::new(opts.max_samples);
    store.load_into(&mut tracker);

    let window = Duration::from_secs(opts.window_secs);
    match tracker.analyze(window) {
        None => format!(
            "Trend: insufficient data (need at least 2 samples within {}s window)",
            opts.window_secs
        ),
        Some(summary) => {
            let dir_str = match summary.direction {
                TrendDirection::Rising => "↑ RISING",
                TrendDirection::Falling => "↓ FALLING",
                TrendDirection::Stable => "→ STABLE",
            };
            format!(
                "Trend [{}s window]: {} | delta={:+} ports | samples={}",
                summary.window_secs, dir_str, summary.delta, summary.samples
            )
        }
    }
}

pub fn format_trend_history(tracker: &TrendTracker) -> Vec<String> {
    tracker
        .history()
        .iter()
        .map(|p| {
            let secs = p
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            format!("  t={} ports={}", secs, p.port_count)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_trend_report_no_data() {
        let opts = TrendCmdOptions {
            store_path: "/tmp/portwatch_nonexistent_trend_cmd.json".to_string(),
            window_secs: 3600,
            max_samples: 50,
        };
        let output = run_trend_report(&opts);
        assert!(output.contains("insufficient data"));
    }

    #[test]
    fn test_format_trend_history_empty() {
        let tracker = TrendTracker::new(10);
        let lines = format_trend_history(&tracker);
        assert!(lines.is_empty());
    }
}

//! CLI sub-commands for inspecting the aggregation state.

use crate::aggregation::Aggregator;
use std::time::Duration;

#[derive(Debug)]
pub enum AggregationCmd {
    Status,
    Flush,
    SetWindow(u64),
}

pub struct AggregationCmdHandler {
    pub aggregator: Aggregator,
    pub window_secs: u64,
}

impl AggregationCmdHandler {
    pub fn new(window_secs: u64) -> Self {
        Self {
            aggregator: Aggregator::new(Duration::from_secs(window_secs)),
            window_secs,
        }
    }

    pub fn handle(&mut self, cmd: AggregationCmd) -> String {
        match cmd {
            AggregationCmd::Status => {
                let groups = self.aggregator.active_groups();
                if groups.is_empty() {
                    return "No active aggregation groups.".to_string();
                }
                let mut lines = vec![format!("Active groups: {}", groups.len())];
                for g in groups {
                    lines.push(format!(
                        "  [{}] count={} sample=\"{}\" last_seen={:?}ms ago",
                        g.key,
                        g.count,
                        g.sample,
                        g.last_seen.elapsed().as_millis()
                    ));
                }
                lines.join("\n")
            }
            AggregationCmd::Flush => {
                let flushed = self.aggregator.flush();
                format!("Flushed {} aggregation group(s).", flushed.len())
            }
            AggregationCmd::SetWindow(secs) => {
                self.window_secs = secs;
                self.aggregator = Aggregator::new(Duration::from_secs(secs));
                format!("Aggregation window set to {}s (state reset).", secs)
            }
        }
    }
}

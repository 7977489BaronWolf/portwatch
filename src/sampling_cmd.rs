//! CLI subcommands for inspecting sampler state.

use crate::sampling::{Sampler, SamplingConfig};
use std::time::Duration;

#[derive(Debug)]
pub enum SamplingCmd {
    Status { key: Option<String> },
    Reset { key: String },
    Configure {
        base_rate: f64,
        high_freq_threshold: u32,
        reduced_rate: f64,
        window_secs: u64,
    },
}

pub struct SamplingCmdHandler {
    pub sampler: Sampler,
}

impl SamplingCmdHandler {
    pub fn new(config: SamplingConfig) -> Self {
        Self {
            sampler: Sampler::new(config),
        }
    }

    pub fn handle(&mut self, cmd: SamplingCmd) -> String {
        match cmd {
            SamplingCmd::Status { key: Some(k) } => match self.sampler.stats(&k) {
                Some((count, sampled)) => format!(
                    "key={} count={} sampled={} drop_rate={:.1}%",
                    k,
                    count,
                    sampled,
                    if count > 0 {
                        (1.0 - sampled as f64 / count as f64) * 100.0
                    } else {
                        0.0
                    }
                ),
                None => format!("No stats for key: {}", k),
            },
            SamplingCmd::Status { key: None } => {
                "Use 'sampling status <key>' to query a specific key.".to_string()
            }
            SamplingCmd::Reset { key } => {
                self.sampler.reset(&key);
                format!("Reset sampler state for key: {}", key)
            }
            SamplingCmd::Configure {
                base_rate,
                high_freq_threshold,
                reduced_rate,
                window_secs,
            } => {
                let config = SamplingConfig {
                    base_rate,
                    high_freq_threshold,
                    reduced_rate,
                    window: Duration::from_secs(window_secs),
                };
                self.sampler = Sampler::new(config);
                format!(
                    "Sampler reconfigured: base_rate={} threshold={} reduced_rate={} window={}s",
                    base_rate, high_freq_threshold, reduced_rate, window_secs
                )
            }
        }
    }
}

//! CLI subcommands for quota inspection and management.

use crate::quota::{QuotaConfig, QuotaManager};
use std::time::Duration;

#[derive(Debug)]
pub enum QuotaCmd {
    Status { channel: Option<String> },
    Reset { channel: String },
    SetLimit { channel: String, max: usize, window_secs: u64 },
}

pub struct QuotaCmdHandler {
    manager: QuotaManager,
}

impl QuotaCmdHandler {
    pub fn new(manager: QuotaManager) -> Self {
        Self { manager }
    }

    pub fn handle(&mut self, cmd: QuotaCmd) -> String {
        match cmd {
            QuotaCmd::Status { channel } => self.status(channel),
            QuotaCmd::Reset { channel } => self.reset(&channel),
            QuotaCmd::SetLimit { channel, max, window_secs } => {
                self.set_limit(&channel, max, window_secs)
            }
        }
    }

    fn status(&self, channel: Option<String>) -> String {
        match channel {
            Some(ch) => {
                let rem = self.manager.remaining(&ch);
                format!("channel='{}' remaining={}", ch, rem)
            }
            None => "Use --channel to query a specific channel quota.".to_string(),
        }
    }

    fn reset(&mut self, channel: &str) -> String {
        self.manager.reset(channel);
        format!("Quota reset for channel='{}'.", channel)
    }

    fn set_limit(&mut self, channel: &str, max: usize, window_secs: u64) -> String {
        self.manager.set_channel_config(
            channel,
            QuotaConfig {
                max_per_window: max,
                window: Duration::from_secs(window_secs),
            },
        );
        format!(
            "Quota configured: channel='{}' max={} window={}s",
            channel, max, window_secs
        )
    }
}

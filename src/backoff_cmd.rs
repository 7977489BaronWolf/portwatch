//! CLI helpers for inspecting and simulating backoff schedules.

use crate::backoff::{Backoff, BackoffConfig};

#[derive(Debug)]
pub struct BackoffSimArgs {
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub max_attempts: u32,
}

impl Default for BackoffSimArgs {
    fn default() -> Self {
        let cfg = BackoffConfig::default();
        Self {
            initial_delay_ms: cfg.initial_delay_ms,
            max_delay_ms: cfg.max_delay_ms,
            multiplier: cfg.multiplier,
            max_attempts: cfg.max_attempts,
        }
    }
}

/// Simulate a full backoff schedule and return each delay in milliseconds.
pub fn simulate_schedule(args: &BackoffSimArgs) -> Vec<u64> {
    let cfg = BackoffConfig {
        initial_delay_ms: args.initial_delay_ms,
        max_delay_ms: args.max_delay_ms,
        multiplier: args.multiplier,
        max_attempts: args.max_attempts,
    };
    let mut backoff = Backoff::new(cfg);
    let mut delays = Vec::new();
    while let Some(d) = backoff.next_delay() {
        delays.push(d.as_millis() as u64);
    }
    delays
}

/// Print a human-readable backoff schedule to stdout.
pub fn print_schedule(args: &BackoffSimArgs) {
    let schedule = simulate_schedule(args);
    println!("Backoff schedule ({} attempts):", schedule.len());
    let mut cumulative_ms: u64 = 0;
    for (i, &ms) in schedule.iter().enumerate() {
        cumulative_ms += ms;
        println!(
            "  Attempt {:>2}: wait {:>6} ms  (cumulative: {:>7} ms)",
            i + 1,
            ms,
            cumulative_ms
        );
    }
    println!("  Total wait before giving up: {} ms", cumulative_ms);
}

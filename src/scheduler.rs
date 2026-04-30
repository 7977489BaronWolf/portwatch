use std::time::{Duration, Instant};
use std::thread;
use crate::config::Config;

/// Scheduler controls the timing of port scan cycles.
pub struct Scheduler {
    interval: Duration,
    last_run: Option<Instant>,
}

impl Scheduler {
    /// Create a new Scheduler from the given config.
    pub fn new(config: &Config) -> Self {
        Scheduler {
            interval: Duration::from_secs(config.scan_interval_secs),
            last_run: None,
        }
    }

    /// Create a Scheduler with an explicit interval (useful for testing).
    pub fn with_interval(interval: Duration) -> Self {
        Scheduler {
            interval,
            last_run: None,
        }
    }

    /// Block until the next scan should run, then record the run time.
    pub fn wait_for_next_tick(&mut self) {
        if let Some(last) = self.last_run {
            let elapsed = last.elapsed();
            if elapsed < self.interval {
                thread::sleep(self.interval - elapsed);
            }
        }
        self.last_run = Some(Instant::now());
    }

    /// Returns true if enough time has passed since the last run.
    pub fn is_due(&self) -> bool {
        match self.last_run {
            None => true,
            Some(last) => last.elapsed() >= self.interval,
        }
    }

    /// Mark the current moment as the last run time without sleeping.
    pub fn mark_run(&mut self) {
        self.last_run = Some(Instant::now());
    }

    /// Return the configured scan interval.
    pub fn interval(&self) -> Duration {
        self.interval
    }
}

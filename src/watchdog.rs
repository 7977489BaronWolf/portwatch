//! Watchdog module: monitors the portwatch daemon's own health and restarts
//! stalled scan cycles if they exceed a configurable timeout.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Shared heartbeat timestamp (Unix seconds), updated by the scan loop.
pub struct Watchdog {
    last_heartbeat: Arc<AtomicI64>,
    timeout_secs: u64,
}

impl Watchdog {
    /// Create a new `Watchdog` with the given stall timeout.
    pub fn new(timeout_secs: u64) -> Self {
        let ts = now_secs();
        Self {
            last_heartbeat: Arc::new(AtomicI64::new(ts)),
            timeout_secs,
        }
    }

    /// Return a cheap handle that the scan loop uses to record heartbeats.
    pub fn handle(&self) -> WatchdogHandle {
        WatchdogHandle {
            last_heartbeat: Arc::clone(&self.last_heartbeat),
        }
    }

    /// Check whether the scan loop is still alive.
    /// Returns `Ok(elapsed_secs)` when healthy, `Err(elapsed_secs)` when stalled.
    pub fn check(&self) -> Result<u64, u64> {
        let last = self.last_heartbeat.load(Ordering::Relaxed);
        let elapsed = (now_secs() - last).max(0) as u64;
        if elapsed <= self.timeout_secs {
            Ok(elapsed)
        } else {
            Err(elapsed)
        }
    }

    /// Reset the heartbeat manually (e.g. on daemon startup).
    pub fn reset(&self) {
        self.last_heartbeat.store(now_secs(), Ordering::Relaxed);
    }

    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }
}

/// Lightweight handle cloned into the scan loop.
#[derive(Clone)]
pub struct WatchdogHandle {
    last_heartbeat: Arc<AtomicI64>,
}

impl WatchdogHandle {
    /// Record a successful heartbeat (call after each completed scan cycle).
    pub fn beat(&self) {
        self.last_heartbeat.store(now_secs(), Ordering::Relaxed);
    }
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs() as i64
}

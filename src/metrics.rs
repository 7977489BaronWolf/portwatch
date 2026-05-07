//! Metrics collection and exposure for portwatch daemon.
//!
//! Tracks runtime statistics such as scan counts, alert counts,
//! and last scan duration for observability purposes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::fmt;

#[derive(Debug, Default)]
pub struct Metrics {
    pub scans_total: AtomicU64,
    pub alerts_total: AtomicU64,
    pub errors_total: AtomicU64,
    pub last_scan_duration_ms: AtomicU64,
    pub ports_open_last: AtomicU64,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn record_scan(&self, duration: Duration, open_ports: u64) {
        self.scans_total.fetch_add(1, Ordering::Relaxed);
        self.last_scan_duration_ms
            .store(duration.as_millis() as u64, Ordering::Relaxed);
        self.ports_open_last.store(open_ports, Ordering::Relaxed);
    }

    pub fn record_alert(&self) {
        self.alerts_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.errors_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            scans_total: self.scans_total.load(Ordering::Relaxed),
            alerts_total: self.alerts_total.load(Ordering::Relaxed),
            errors_total: self.errors_total.load(Ordering::Relaxed),
            last_scan_duration_ms: self.last_scan_duration_ms.load(Ordering::Relaxed),
            ports_open_last: self.ports_open_last.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricsSnapshot {
    pub scans_total: u64,
    pub alerts_total: u64,
    pub errors_total: u64,
    pub last_scan_duration_ms: u64,
    pub ports_open_last: u64,
}

impl fmt::Display for MetricsSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "scans={} alerts={} errors={} last_scan_ms={} open_ports={}",
            self.scans_total,
            self.alerts_total,
            self.errors_total,
            self.last_scan_duration_ms,
            self.ports_open_last
        )
    }
}

/// Time a block and record scan metrics.
pub fn timed_scan<F: FnOnce() -> u64>(metrics: &Metrics, f: F) -> u64 {
    let start = Instant::now();
    let open_ports = f();
    metrics.record_scan(start.elapsed(), open_ports);
    open_ports
}

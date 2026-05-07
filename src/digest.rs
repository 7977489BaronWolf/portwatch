//! Digest module: computes and tracks a fingerprint of the current port state
//! so that unchanged scans can be skipped quickly without a full diff.

use std::collections::BTreeSet;
use std::fmt;
use sha2::{Digest as Sha2Digest, Sha256};

/// A compact, comparable fingerprint of a port-state snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateDigest(pub String);

impl fmt::Display for StateDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Compute a deterministic SHA-256 digest over a sorted list of port entries.
///
/// Each entry is expected to be a canonical string such as `"tcp:8080"` or
/// `"udp:53"`.  Using a `BTreeSet` guarantees ordering is independent of the
/// order in which the scanner returns results.
pub fn compute_digest(ports: &[String]) -> StateDigest {
    let sorted: BTreeSet<&String> = ports.iter().collect();
    let mut hasher = Sha256::new();
    for entry in &sorted {
        hasher.update(entry.as_bytes());
        hasher.update(b"\n");
    }
    let result = hasher.finalize();
    StateDigest(format!("{:x}", result))
}

/// Returns `true` when the two digests differ, indicating that the port state
/// has changed since the last scan cycle.
pub fn has_changed(previous: &StateDigest, current: &StateDigest) -> bool {
    previous != current
}

/// A small cache that holds the last known digest so callers do not need to
/// store it themselves.
#[derive(Default)]
pub struct DigestCache {
    last: Option<StateDigest>,
}

impl DigestCache {
    pub fn new() -> Self {
        Self { last: None }
    }

    /// Update the cache with `current` and return whether the state changed.
    /// Returns `true` on the very first call (no previous baseline).
    pub fn update(&mut self, current: StateDigest) -> bool {
        let changed = match &self.last {
            None => true,
            Some(prev) => has_changed(prev, &current),
        };
        self.last = Some(current);
        changed
    }

    pub fn last(&self) -> Option<&StateDigest> {
        self.last.as_ref()
    }
}

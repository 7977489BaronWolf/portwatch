//! Retention policy for audit logs, snapshots, and state history.
//! Enforces configurable max-age and max-count limits.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Maximum number of files to keep (0 = unlimited)
    pub max_count: usize,
    /// Maximum age of files to keep (None = unlimited)
    pub max_age: Option<Duration>,
}

impl RetentionPolicy {
    pub fn new(max_count: usize, max_age_days: Option<u64>) -> Self {
        Self {
            max_count,
            max_age: max_age_days.map(|d| Duration::from_secs(d * 86_400)),
        }
    }

    /// Apply the retention policy to all files in `dir` matching `prefix`.
    /// Returns the number of files removed.
    pub fn apply(&self, dir: &Path, prefix: &str) -> std::io::Result<usize> {
        let mut entries: Vec<(PathBuf, SystemTime)> = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(prefix)
            })
            .filter_map(|e| {
                let modified = e.metadata().ok()?.modified().ok()?;
                Some((e.path(), modified))
            })
            .collect();

        // Sort oldest first
        entries.sort_by_key(|(_, mtime)| *mtime);

        let now = SystemTime::now();
        let mut removed = 0;

        // Remove files exceeding max_age
        if let Some(max_age) = self.max_age {
            entries.retain(|(path, mtime)| {
                if now.duration_since(*mtime).unwrap_or(Duration::ZERO) > max_age {
                    let _ = fs::remove_file(path);
                    removed += 1;
                    false
                } else {
                    true
                }
            });
        }

        // Remove oldest files exceeding max_count
        if self.max_count > 0 && entries.len() > self.max_count {
            let excess = entries.len() - self.max_count;
            for (path, _) in entries.iter().take(excess) {
                let _ = fs::remove_file(path);
                removed += 1;
            }
        }

        Ok(removed)
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self::new(30, Some(90))
    }
}

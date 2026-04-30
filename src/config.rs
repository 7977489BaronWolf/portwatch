use std::fs;
use std::path::Path;
use serde::Deserialize;

/// Top-level configuration for portwatch.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// How often (in seconds) to scan open ports.
    pub scan_interval_secs: u64,

    /// Path to the state file used for persisting port snapshots.
    pub state_file: String,

    /// Optional list of ports to ignore during diffing.
    #[serde(default)]
    pub ignored_ports: Vec<u16>,

    /// Notification hooks (e.g. webhook URLs or script paths).
    #[serde(default)]
    pub notification_hooks: Vec<String>,

    /// Log level: "info", "warn", "error", "debug".
    #[serde(default = "Config::default_log_level")]
    pub log_level: String,
}

impl Config {
    /// Load configuration from a TOML file at the given path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    /// Return a sensible default configuration for testing or quick-start.
    pub fn default_config() -> Self {
        Config {
            scan_interval_secs: 60,
            state_file: "/var/lib/portwatch/state.json".to_string(),
            ignored_ports: vec![],
            notification_hooks: vec![],
            log_level: Self::default_log_level(),
        }
    }

    fn default_log_level() -> String {
        "info".to_string()
    }

    /// Validate configuration values and return an error string if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.scan_interval_secs == 0 {
            return Err("scan_interval_secs must be greater than 0".to_string());
        }
        if self.state_file.is_empty() {
            return Err("state_file path must not be empty".to_string());
        }
        let valid_levels = ["debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(format!("Invalid log_level '{}'", self.log_level));
        }
        Ok(())
    }
}

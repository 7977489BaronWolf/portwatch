use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub scan_interval_secs: u64,
    pub ports_to_watch: Vec<u16>,
    pub notification_hooks: Vec<NotificationHook>,
    pub log_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationHook {
    pub name: String,
    pub hook_type: HookType,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookType {
    Webhook,
    Command,
    Email,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            scan_interval_secs: 30,
            ports_to_watch: vec![],
            notification_hooks: vec![],
            log_file: None,
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        if !Path::new(path).exists() {
            return Err(ConfigError::FileNotFound(path.to_string()));
        }
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.scan_interval_secs == 0 {
            return Err(ConfigError::InvalidValue(
                "scan_interval_secs must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ReadError(String),
    ParseError(String),
    InvalidValue(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(p) => write!(f, "Config file not found: {}", p),
            ConfigError::ReadError(e) => write!(f, "Failed to read config: {}", e),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config: {}", e),
            ConfigError::InvalidValue(e) => write!(f, "Invalid config value: {}", e),
        }
    }
}

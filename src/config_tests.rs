#[cfg(test)]
mod tests {
    use super::super::config::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_config(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("failed to create temp file");
        file.write_all(content.as_bytes()).expect("failed to write config");
        file
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.scan_interval_secs, 30);
        assert!(config.ports_to_watch.is_empty());
        assert!(config.notification_hooks.is_empty());
        assert!(config.log_file.is_none());
    }

    #[test]
    fn test_load_valid_config() {
        let content = r#"
            scan_interval_secs = 60
            ports_to_watch = [80, 443, 8080]
            log_file = "/var/log/portwatch.log"

            [[notification_hooks]]
            name = "slack"
            hook_type = "webhook"
            target = "https://hooks.slack.com/test"
        "#;
        let file = write_temp_config(content);
        let config = Config::load(file.path().to_str().unwrap()).unwrap();
        assert_eq!(config.scan_interval_secs, 60);
        assert_eq!(config.ports_to_watch, vec![80, 443, 8080]);
        assert_eq!(config.notification_hooks.len(), 1);
        assert_eq!(config.notification_hooks[0].name, "slack");
    }

    #[test]
    fn test_load_missing_file() {
        let result = Config::load("/nonexistent/path/portwatch.toml");
        assert!(matches!(result, Err(ConfigError::FileNotFound(_))));
    }

    #[test]
    fn test_load_invalid_toml() {
        let file = write_temp_config("this is not valid toml !!!");
        let result = Config::load(file.path().to_str().unwrap());
        assert!(matches!(result, Err(ConfigError::ParseError(_))));
    }

    #[test]
    fn test_validate_zero_interval() {
        let mut config = Config::default();
        config.scan_interval_secs = 0;
        let result = config.validate();
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_validate_valid_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }
}

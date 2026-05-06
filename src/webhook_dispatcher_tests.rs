#[cfg(test)]
mod tests {
    use super::super::webhook::WebhookConfig;
    use super::super::webhook_dispatcher::WebhookDispatcher;

    fn valid_config(url: &str) -> WebhookConfig {
        WebhookConfig {
            url: url.to_string(),
            retry_count: 0,
            ..Default::default()
        }
    }

    fn invalid_config() -> WebhookConfig {
        WebhookConfig {
            url: "".to_string(),
            retry_count: 0,
            ..Default::default()
        }
    }

    #[test]
    fn test_dispatcher_no_clients() {
        let dispatcher = WebhookDispatcher::new(vec![]);
        assert_eq!(dispatcher.client_count(), 0);
        let results = dispatcher.dispatch("test_event", serde_json::json!({}));
        assert!(results.is_empty());
    }

    #[test]
    fn test_dispatcher_single_valid_client() {
        let dispatcher = WebhookDispatcher::new(vec![valid_config("https://hooks.example.com/a")]);
        assert_eq!(dispatcher.client_count(), 1);
        let results = dispatcher.dispatch("port_opened", serde_json::json!({"port": 443}));
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
    }

    #[test]
    fn test_dispatcher_multiple_clients() {
        let dispatcher = WebhookDispatcher::new(vec![
            valid_config("https://hooks.example.com/a"),
            valid_config("https://hooks.example.com/b"),
        ]);
        assert_eq!(dispatcher.client_count(), 2);
        let results = dispatcher.dispatch("port_closed", serde_json::json!({"port": 22}));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_dispatch_records_logged() {
        let dispatcher = WebhookDispatcher::new(vec![valid_config("https://hooks.example.com/log")]);
        dispatcher.dispatch("port_opened", serde_json::json!({"port": 8080}));
        let records = dispatcher.records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].event, "port_opened");
        assert!(records[0].success);
    }

    #[test]
    fn test_dispatch_failed_client_logged() {
        let dispatcher = WebhookDispatcher::new(vec![invalid_config()]);
        dispatcher.dispatch("port_opened", serde_json::json!({}));
        let records = dispatcher.records();
        assert_eq!(records.len(), 1);
        assert!(!records[0].success);
    }

    #[test]
    fn test_dispatch_port_change_helper() {
        let dispatcher = WebhookDispatcher::new(vec![valid_config("https://hooks.example.com/pc")]);
        let results = dispatcher.dispatch_port_change(9090, "tcp", "port_opened");
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        let records = dispatcher.records();
        assert_eq!(records[0].event, "port_opened");
    }

    #[test]
    fn test_multiple_dispatches_accumulate_records() {
        let dispatcher = WebhookDispatcher::new(vec![valid_config("https://hooks.example.com/multi")]);
        dispatcher.dispatch("port_opened", serde_json::json!({"port": 80}));
        dispatcher.dispatch("port_closed", serde_json::json!({"port": 80}));
        let records = dispatcher.records();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].event, "port_opened");
        assert_eq!(records[1].event, "port_closed");
    }
}

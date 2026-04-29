#[cfg(test)]
mod tests {
    use super::super::config::{HookType, NotificationHook};
    use super::super::notifier::Notifier;
    use super::super::port_scanner::PortChange;

    fn make_hook(name: &str, hook_type: HookType, target: &str) -> NotificationHook {
        NotificationHook {
            name: name.to_string(),
            hook_type,
            target: target.to_string(),
        }
    }

    fn sample_changes() -> Vec<PortChange> {
        vec![
            PortChange::Opened(8080),
            PortChange::Closed(3000),
        ]
    }

    #[test]
    fn test_notify_all_no_changes_returns_no_errors() {
        let notifier = Notifier::new(vec![make_hook("test", HookType::Command, "true")]);
        let errors = notifier.notify_all(&[]);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_notify_all_with_command_hook_success() {
        let notifier = Notifier::new(vec![make_hook("echo", HookType::Command, "true")]);
        let errors = notifier.notify_all(&sample_changes());
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_notify_all_with_failing_command() {
        let notifier = Notifier::new(vec![make_hook("fail", HookType::Command, "false")]);
        let errors = notifier.notify_all(&sample_changes());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].to_string().contains("Command"));
    }

    #[test]
    fn test_notify_all_email_unsupported() {
        let notifier = Notifier::new(vec![make_hook("mail", HookType::Email, "user@example.com")]);
        let errors = notifier.notify_all(&sample_changes());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].to_string().contains("Unsupported"));
    }

    #[test]
    fn test_notify_all_multiple_hooks_collects_all_errors() {
        let notifier = Notifier::new(vec![
            make_hook("fail1", HookType::Command, "false"),
            make_hook("fail2", HookType::Email, "user@example.com"),
            make_hook("ok", HookType::Command, "true"),
        ]);
        let errors = notifier.notify_all(&sample_changes());
        assert_eq!(errors.len(), 2);
    }
}

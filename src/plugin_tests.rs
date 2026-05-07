#[cfg(test)]
mod tests {
    use crate::plugin::{PluginHook, PluginMeta, PluginRegistry};

    fn make_meta(name: &str, hooks: Vec<PluginHook>) -> PluginMeta {
        PluginMeta {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            hooks,
        }
    }

    #[test]
    fn test_register_and_count() {
        let mut reg = PluginRegistry::new();
        reg.register(make_meta("slack", vec![PluginHook::OnAlert]));
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn test_plugins_for_hook_returns_registered() {
        let mut reg = PluginRegistry::new();
        reg.register(make_meta("pagerduty", vec![PluginHook::OnAlert, PluginHook::OnPortOpen]));
        let alert_plugins = reg.plugins_for_hook(&PluginHook::OnAlert);
        assert!(alert_plugins.contains(&"pagerduty"));
    }

    #[test]
    fn test_plugins_for_hook_empty_when_none() {
        let reg = PluginRegistry::new();
        let result = reg.plugins_for_hook(&PluginHook::OnPortClose);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_plugins_same_hook() {
        let mut reg = PluginRegistry::new();
        reg.register(make_meta("plugin_a", vec![PluginHook::OnBaselineViolation]));
        reg.register(make_meta("plugin_b", vec![PluginHook::OnBaselineViolation]));
        let plugins = reg.plugins_for_hook(&PluginHook::OnBaselineViolation);
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"plugin_a"));
        assert!(plugins.contains(&"plugin_b"));
    }

    #[test]
    fn test_get_plugin_meta() {
        let mut reg = PluginRegistry::new();
        reg.register(make_meta("opsgenie", vec![PluginHook::OnPortClose]));
        let meta = reg.get("opsgenie").expect("plugin should exist");
        assert_eq!(meta.version, "1.0.0");
        assert_eq!(meta.hooks, vec![PluginHook::OnPortClose]);
    }

    #[test]
    fn test_unregister_removes_plugin() {
        let mut reg = PluginRegistry::new();
        reg.register(make_meta("temp", vec![PluginHook::OnAlert]));
        assert!(reg.unregister("temp"));
        assert_eq!(reg.count(), 0);
        assert!(reg.plugins_for_hook(&PluginHook::OnAlert).is_empty());
    }

    #[test]
    fn test_unregister_nonexistent_returns_false() {
        let mut reg = PluginRegistry::new();
        assert!(!reg.unregister("ghost"));
    }

    #[test]
    fn test_hook_display() {
        assert_eq!(PluginHook::OnPortOpen.to_string(), "on_port_open");
        assert_eq!(PluginHook::OnBaselineViolation.to_string(), "on_baseline_violation");
    }
}

#[cfg(test)]
mod tests {
    use crate::health::{ComponentHealth, HealthRegistry, HealthStatus};

    #[test]
    fn test_component_health_ok() {
        let c = ComponentHealth::new("scanner", HealthStatus::Ok);
        assert!(c.is_healthy());
        assert_eq!(c.name, "scanner");
    }

    #[test]
    fn test_component_health_degraded() {
        let c = ComponentHealth::new("notifier", HealthStatus::Degraded("slow".into()));
        assert!(!c.is_healthy());
    }

    #[test]
    fn test_component_health_failed() {
        let c = ComponentHealth::new("state", HealthStatus::Failed("disk full".into()));
        assert!(!c.is_healthy());
    }

    #[test]
    fn test_registry_overall_ok() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("a", HealthStatus::Ok));
        reg.register(ComponentHealth::new("b", HealthStatus::Ok));
        assert_eq!(reg.overall_status(), HealthStatus::Ok);
    }

    #[test]
    fn test_registry_overall_degraded() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("a", HealthStatus::Ok));
        reg.register(ComponentHealth::new("b", HealthStatus::Degraded("slow".into())));
        match reg.overall_status() {
            HealthStatus::Degraded(msg) => assert!(msg.contains("slow")),
            other => panic!("Expected Degraded, got {:?}", other),
        }
    }

    #[test]
    fn test_registry_overall_failed_takes_priority() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("a", HealthStatus::Degraded("warn".into())));
        reg.register(ComponentHealth::new("b", HealthStatus::Failed("crash".into())));
        match reg.overall_status() {
            HealthStatus::Failed(msg) => assert!(msg.contains("crash")),
            other => panic!("Expected Failed, got {:?}", other),
        }
    }

    #[test]
    fn test_registry_update() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("scanner", HealthStatus::Ok));
        reg.update("scanner", HealthStatus::Degraded("timeout".into()));
        let c = reg.get("scanner").unwrap();
        assert!(!c.is_healthy());
    }

    #[test]
    fn test_registry_get_missing() {
        let reg = HealthRegistry::new();
        assert!(reg.get("nonexistent").is_none());
    }

    #[test]
    fn test_registry_components_list() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("x", HealthStatus::Ok));
        reg.register(ComponentHealth::new("y", HealthStatus::Ok));
        assert_eq!(reg.components().len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use crate::health::{ComponentHealth, HealthRegistry, HealthStatus};
    use crate::health_cmd::{run_health_cmd, HealthCmdOptions};

    fn make_registry() -> HealthRegistry {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("scanner", HealthStatus::Ok));
        reg.register(ComponentHealth::new(
            "notifier",
            HealthStatus::Degraded("slow webhook".into()),
        ));
        reg.register(ComponentHealth::new(
            "state_store",
            HealthStatus::Failed("disk error".into()),
        ));
        reg
    }

    #[test]
    fn test_text_output_contains_overall() {
        let reg = make_registry();
        let opts = HealthCmdOptions { failures_only: false, json: false };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("Overall:"));
        assert!(out.contains("FAILED"));
    }

    #[test]
    fn test_text_output_all_components() {
        let reg = make_registry();
        let opts = HealthCmdOptions { failures_only: false, json: false };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("scanner"));
        assert!(out.contains("notifier"));
        assert!(out.contains("state_store"));
    }

    #[test]
    fn test_failures_only_hides_ok() {
        let reg = make_registry();
        let opts = HealthCmdOptions { failures_only: true, json: false };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(!out.contains("[OK] scanner"));
        assert!(out.contains("notifier") || out.contains("state_store"));
    }

    #[test]
    fn test_json_output_structure() {
        let reg = make_registry();
        let opts = HealthCmdOptions { failures_only: false, json: true };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("\"overall\""));
        assert!(out.contains("\"components\""));
        assert!(out.contains("\"name\""));
        assert!(out.contains("\"status\""));
    }

    #[test]
    fn test_json_contains_detail() {
        let reg = make_registry();
        let opts = HealthCmdOptions { failures_only: false, json: true };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("disk error"));
        assert!(out.contains("slow webhook"));
    }

    #[test]
    fn test_all_ok_registry() {
        let mut reg = HealthRegistry::new();
        reg.register(ComponentHealth::new("a", HealthStatus::Ok));
        let opts = HealthCmdOptions { failures_only: false, json: false };
        let mut buf = Vec::new();
        run_health_cmd(&reg, &opts, &mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("Overall: OK"));
    }
}

#[cfg(test)]
mod tests {
    use super::super::routing::{Router, RoutingRule};

    fn make_rule(name: &str, severity: Option<&str>, tag: Option<&str>, channels: &[&str]) -> RoutingRule {
        RoutingRule {
            name: name.to_string(),
            match_severity: severity.map(|s| s.to_string()),
            match_tag: tag.map(|t| t.to_string()),
            channels: channels.iter().map(|c| c.to_string()).collect(),
        }
    }

    #[test]
    fn test_empty_router_returns_no_channels() {
        let router = Router::new();
        let result = router.resolve("high", &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_rule_matches_any_severity_when_unset() {
        let mut router = Router::new();
        router.add_rule(make_rule("catch-all", None, None, &["slack"]));
        let result = router.resolve("low", &[]);
        assert_eq!(result, vec!["slack"]);
    }

    #[test]
    fn test_rule_matches_specific_severity() {
        let mut router = Router::new();
        router.add_rule(make_rule("high-only", Some("high"), None, &["pagerduty"]));
        let high = router.resolve("high", &[]);
        let low = router.resolve("low", &[]);
        assert_eq!(high, vec!["pagerduty"]);
        assert!(low.is_empty());
    }

    #[test]
    fn test_rule_matches_tag() {
        let mut router = Router::new();
        router.add_rule(make_rule("db-tag", None, Some("database"), &["email"]));
        let with_tag = router.resolve("medium", &["database".to_string()]);
        let without_tag = router.resolve("medium", &["network".to_string()]);
        assert_eq!(with_tag, vec!["email"]);
        assert!(without_tag.is_empty());
    }

    #[test]
    fn test_multiple_rules_deduplicate_channels() {
        let mut router = Router::new();
        router.add_rule(make_rule("r1", Some("high"), None, &["slack"]));
        router.add_rule(make_rule("r2", None, None, &["slack", "email"]));
        let result = router.resolve("high", &[]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"slack".to_string()));
        assert!(result.contains(&"email".to_string()));
    }

    #[test]
    fn test_rule_count() {
        let mut router = Router::new();
        assert_eq!(router.rule_count(), 0);
        router.add_rule(make_rule("r1", None, None, &["slack"]));
        router.add_rule(make_rule("r2", Some("high"), None, &["pagerduty"]));
        assert_eq!(router.rule_count(), 2);
    }

    #[test]
    fn test_severity_and_tag_both_must_match() {
        let mut router = Router::new();
        router.add_rule(make_rule("strict", Some("critical"), Some("firewall"), &["oncall"]));
        let both = router.resolve("critical", &["firewall".to_string()]);
        let only_sev = router.resolve("critical", &[]);
        let only_tag = router.resolve("low", &["firewall".to_string()]);
        assert_eq!(both, vec!["oncall"]);
        assert!(only_sev.is_empty());
        assert!(only_tag.is_empty());
    }
}

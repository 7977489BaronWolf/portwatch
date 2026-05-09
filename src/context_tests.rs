#[cfg(test)]
mod tests {
    use super::super::context::PortContext;

    #[test]
    fn test_new_context_defaults() {
        let ctx = PortContext::new(8080, "tcp");
        assert_eq!(ctx.port, 8080);
        assert_eq!(ctx.protocol, "tcp");
        assert!(ctx.pid.is_none());
        assert!(ctx.process_name.is_none());
        assert!(ctx.username.is_none());
        assert!(ctx.extra.is_empty());
    }

    #[test]
    fn test_builder_chain() {
        let ctx = PortContext::new(443, "tcp")
            .with_pid(1234)
            .with_process("nginx")
            .with_user("www-data");
        assert_eq!(ctx.pid, Some(1234));
        assert_eq!(ctx.process_name.as_deref(), Some("nginx"));
        assert_eq!(ctx.username.as_deref(), Some("www-data"));
    }

    #[test]
    fn test_insert_extra() {
        let mut ctx = PortContext::new(22, "tcp");
        ctx.insert_extra("iface", "eth0");
        ctx.insert_extra("region", "us-east-1");
        assert_eq!(ctx.extra.get("iface").map(|s| s.as_str()), Some("eth0"));
        assert_eq!(ctx.extra.get("region").map(|s| s.as_str()), Some("us-east-1"));
    }

    #[test]
    fn test_summary_with_all_fields() {
        let ctx = PortContext::new(80, "tcp")
            .with_process("apache2")
            .with_user("root");
        let s = ctx.summary();
        assert!(s.contains("tcp:80"));
        assert!(s.contains("apache2"));
        assert!(s.contains("root"));
    }

    #[test]
    fn test_summary_unknown_fallbacks() {
        let ctx = PortContext::new(9000, "udp");
        let s = ctx.summary();
        assert!(s.contains("udp:9000"));
        assert!(s.contains("unknown"));
    }

    #[test]
    fn test_equality() {
        let a = PortContext::new(8080, "tcp").with_pid(42);
        let b = PortContext::new(8080, "tcp").with_pid(42);
        assert_eq!(a, b);
    }

    #[test]
    fn test_inequality_different_port() {
        let a = PortContext::new(80, "tcp");
        let b = PortContext::new(81, "tcp");
        assert_ne!(a, b);
    }
}

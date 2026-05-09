#[cfg(test)]
mod tests {
    use super::super::context_builder::ContextBuilder;

    #[test]
    fn test_build_returns_context_with_correct_port() {
        let builder = ContextBuilder::new().with_proc(false);
        let ctx = builder.build(8080, "tcp");
        assert_eq!(ctx.port, 8080);
        assert_eq!(ctx.protocol, "tcp");
    }

    #[test]
    fn test_build_no_proc_leaves_pid_none() {
        let builder = ContextBuilder::new().with_proc(false);
        let ctx = builder.build(443, "tcp");
        assert!(ctx.pid.is_none());
        assert!(ctx.process_name.is_none());
    }

    #[test]
    fn test_default_builder() {
        let builder = ContextBuilder::default();
        let ctx = builder.build(22, "tcp");
        assert_eq!(ctx.port, 22);
    }

    #[test]
    fn test_build_udp_protocol() {
        let builder = ContextBuilder::new().with_proc(false);
        let ctx = builder.build(53, "udp");
        assert_eq!(ctx.protocol, "udp");
        assert_eq!(ctx.port, 53);
    }

    #[test]
    fn test_builder_chaining() {
        let ctx = ContextBuilder::new()
            .with_proc(false)
            .build(9090, "tcp");
        assert_eq!(ctx.port, 9090);
    }

    #[test]
    fn test_hostname_set_in_context() {
        let builder = ContextBuilder::new().with_proc(false);
        let ctx = builder.build(80, "tcp");
        // hostname should be non-empty (either env var or "localhost")
        assert!(!ctx.hostname.is_empty());
    }
}

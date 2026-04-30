#[cfg(test)]
mod tests {
    use crate::filter::{FilterRule, FilterSet, Protocol};

    fn tcp_rule(port: u16) -> FilterRule {
        FilterRule {
            port: Some(port),
            port_range: None,
            protocol: Protocol::Tcp,
            comment: None,
        }
    }

    fn range_rule(lo: u16, hi: u16, proto: Protocol) -> FilterRule {
        FilterRule {
            port: None,
            port_range: Some((lo, hi)),
            protocol: proto,
            comment: None,
        }
    }

    #[test]
    fn test_exact_port_match() {
        let rule = tcp_rule(22);
        assert!(rule.matches(22, &Protocol::Tcp));
        assert!(!rule.matches(23, &Protocol::Tcp));
        assert!(!rule.matches(22, &Protocol::Udp));
    }

    #[test]
    fn test_range_match() {
        let rule = range_rule(8000, 8080, Protocol::Tcp);
        assert!(rule.matches(8000, &Protocol::Tcp));
        assert!(rule.matches(8080, &Protocol::Tcp));
        assert!(rule.matches(8040, &Protocol::Tcp));
        assert!(!rule.matches(7999, &Protocol::Tcp));
        assert!(!rule.matches(8081, &Protocol::Tcp));
    }

    #[test]
    fn test_protocol_any_matches_both() {
        let rule = FilterRule {
            port: Some(53),
            port_range: None,
            protocol: Protocol::Any,
            comment: None,
        };
        assert!(rule.matches(53, &Protocol::Tcp));
        assert!(rule.matches(53, &Protocol::Udp));
    }

    #[test]
    fn test_filter_set_is_ignored() {
        let set = FilterSet::new(vec![tcp_rule(22), tcp_rule(80)]);
        assert!(set.is_ignored(22, &Protocol::Tcp));
        assert!(set.is_ignored(80, &Protocol::Tcp));
        assert!(!set.is_ignored(443, &Protocol::Tcp));
    }

    #[test]
    fn test_filter_set_apply() {
        let set = FilterSet::new(vec![tcp_rule(22)]);
        let ports = vec![
            (22, Protocol::Tcp),
            (80, Protocol::Tcp),
            (443, Protocol::Tcp),
        ];
        let result = set.apply(&ports);
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|(p, _)| *p != 22));
    }

    #[test]
    fn test_empty_filter_set_passes_all() {
        let set = FilterSet::default();
        let ports = vec![(22, Protocol::Tcp), (443, Protocol::Udp)];
        let result = set.apply(&ports);
        assert_eq!(result.len(), 2);
    }
}

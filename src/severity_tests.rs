#[cfg(test)]
mod tests {
    use super::super::severity::Severity;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(Severity::from_str("info"), Some(Severity::Info));
        assert_eq!(Severity::from_str("LOW"), Some(Severity::Low));
        assert_eq!(Severity::from_str("Medium"), Some(Severity::Medium));
        assert_eq!(Severity::from_str("high"), Some(Severity::High));
        assert_eq!(Severity::from_str("CRITICAL"), Some(Severity::Critical));
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(Severity::from_str("unknown"), None);
        assert_eq!(Severity::from_str(""), None);
    }

    #[test]
    fn test_ordering() {
        assert!(Severity::Info < Severity::Low);
        assert!(Severity::Low < Severity::Medium);
        assert!(Severity::Medium < Severity::High);
        assert!(Severity::High < Severity::Critical);
    }

    #[test]
    fn test_numeric_score() {
        assert_eq!(Severity::Info.numeric_score(), 0);
        assert_eq!(Severity::Critical.numeric_score(), 4);
    }

    #[test]
    fn test_is_actionable() {
        assert!(!Severity::Info.is_actionable());
        assert!(!Severity::Low.is_actionable());
        assert!(!Severity::Medium.is_actionable());
        assert!(Severity::High.is_actionable());
        assert!(Severity::Critical.is_actionable());
    }

    #[test]
    fn test_escalated() {
        assert_eq!(Severity::Info.escalated(), Some(Severity::Low));
        assert_eq!(Severity::High.escalated(), Some(Severity::Critical));
        assert_eq!(Severity::Critical.escalated(), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Severity::Medium), "medium");
        assert_eq!(format!("{}", Severity::Critical), "critical");
    }

    #[test]
    fn test_default() {
        assert_eq!(Severity::default(), Severity::Info);
    }

    #[test]
    fn test_serialization() {
        let s = Severity::High;
        let json = serde_json::to_string(&s).unwrap();
        let back: Severity = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}

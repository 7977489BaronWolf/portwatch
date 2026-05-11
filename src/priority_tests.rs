#[cfg(test)]
mod tests {
    use super::super::priority::Priority;

    #[test]
    fn test_from_score_low() {
        assert_eq!(Priority::from_score(0), Priority::Low);
        assert_eq!(Priority::from_score(25), Priority::Low);
    }

    #[test]
    fn test_from_score_medium() {
        assert_eq!(Priority::from_score(26), Priority::Medium);
        assert_eq!(Priority::from_score(50), Priority::Medium);
    }

    #[test]
    fn test_from_score_high() {
        assert_eq!(Priority::from_score(51), Priority::High);
        assert_eq!(Priority::from_score(75), Priority::High);
    }

    #[test]
    fn test_from_score_critical() {
        assert_eq!(Priority::from_score(76), Priority::Critical);
        assert_eq!(Priority::from_score(100), Priority::Critical);
    }

    #[test]
    fn test_score_values() {
        assert!(Priority::Low.score() < Priority::Medium.score());
        assert!(Priority::Medium.score() < Priority::High.score());
        assert!(Priority::High.score() < Priority::Critical.score());
    }

    #[test]
    fn test_is_actionable() {
        assert!(!Priority::Low.is_actionable());
        assert!(!Priority::Medium.is_actionable());
        assert!(Priority::High.is_actionable());
        assert!(Priority::Critical.is_actionable());
    }

    #[test]
    fn test_escalate() {
        assert_eq!(Priority::Low.escalate(), Priority::Medium);
        assert_eq!(Priority::Medium.escalate(), Priority::High);
        assert_eq!(Priority::High.escalate(), Priority::Critical);
        assert_eq!(Priority::Critical.escalate(), Priority::Critical);
    }

    #[test]
    fn test_downgrade() {
        assert_eq!(Priority::Critical.downgrade(), Priority::High);
        assert_eq!(Priority::High.downgrade(), Priority::Medium);
        assert_eq!(Priority::Medium.downgrade(), Priority::Low);
        assert_eq!(Priority::Low.downgrade(), Priority::Low);
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(Priority::try_from("low").unwrap(), Priority::Low);
        assert_eq!(Priority::try_from("HIGH").unwrap(), Priority::High);
        assert_eq!(Priority::try_from("Critical").unwrap(), Priority::Critical);
        assert!(Priority::try_from("unknown").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Priority::High), "high");
        assert_eq!(format!("{}", Priority::Critical), "critical");
    }

    #[test]
    fn test_ordering() {
        assert!(Priority::Low < Priority::Medium);
        assert!(Priority::Medium < Priority::High);
        assert!(Priority::High < Priority::Critical);
    }
}

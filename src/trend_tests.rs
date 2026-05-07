#[cfg(test)]
mod tests {
    use super::super::trend::*;
    use std::time::Duration;

    #[test]
    fn test_record_and_history_length() {
        let mut tracker = TrendTracker::new(5);
        for i in 0..7 {
            tracker.record(i * 2);
        }
        assert_eq!(tracker.history().len(), 5);
    }

    #[test]
    fn test_analyze_requires_two_samples() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(10);
        let result = tracker.analyze(Duration::from_secs(3600));
        assert!(result.is_none());
    }

    #[test]
    fn test_rising_trend() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(5);
        tracker.record(10);
        tracker.record(15);
        let summary = tracker.analyze(Duration::from_secs(3600)).unwrap();
        assert_eq!(summary.direction, TrendDirection::Rising);
        assert!(summary.delta > 0);
    }

    #[test]
    fn test_falling_trend() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(20);
        tracker.record(10);
        tracker.record(5);
        let summary = tracker.analyze(Duration::from_secs(3600)).unwrap();
        assert_eq!(summary.direction, TrendDirection::Falling);
        assert!(summary.delta < 0);
    }

    #[test]
    fn test_stable_trend() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(8);
        tracker.record(8);
        tracker.record(8);
        let summary = tracker.analyze(Duration::from_secs(3600)).unwrap();
        assert_eq!(summary.direction, TrendDirection::Stable);
        assert_eq!(summary.delta, 0);
    }

    #[test]
    fn test_samples_count_in_summary() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(1);
        tracker.record(2);
        tracker.record(3);
        let summary = tracker.analyze(Duration::from_secs(3600)).unwrap();
        assert_eq!(summary.samples, 3);
    }

    #[test]
    fn test_max_samples_enforced() {
        let mut tracker = TrendTracker::new(3);
        for i in 0..10 {
            tracker.record(i);
        }
        assert!(tracker.history().len() <= 3);
    }

    #[test]
    fn test_window_secs_in_summary() {
        let mut tracker = TrendTracker::new(10);
        tracker.record(5);
        tracker.record(10);
        let summary = tracker.analyze(Duration::from_secs(600)).unwrap();
        assert_eq!(summary.window_secs, 600);
    }
}

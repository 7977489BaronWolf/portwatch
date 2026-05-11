#[cfg(test)]
mod tests {
    use super::super::priority::Priority;
    use super::super::priority_queue::{PriorityItem, PriorityQueue};

    fn make_item(id: &str, priority: Priority) -> PriorityItem {
        PriorityItem::new(id, priority, format!("payload for {}", id))
    }

    #[test]
    fn test_empty_queue() {
        let q = PriorityQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn test_push_and_pop_single() {
        let mut q = PriorityQueue::new();
        q.push(make_item("a", Priority::Medium));
        assert_eq!(q.len(), 1);
        let item = q.pop().unwrap();
        assert_eq!(item.id, "a");
        assert!(q.is_empty());
    }

    #[test]
    fn test_pop_respects_priority_order() {
        let mut q = PriorityQueue::new();
        q.push(make_item("low", Priority::Low));
        q.push(make_item("critical", Priority::Critical));
        q.push(make_item("medium", Priority::Medium));
        q.push(make_item("high", Priority::High));

        assert_eq!(q.pop().unwrap().id, "critical");
        assert_eq!(q.pop().unwrap().id, "high");
        assert_eq!(q.pop().unwrap().id, "medium");
        assert_eq!(q.pop().unwrap().id, "low");
        assert!(q.pop().is_none());
    }

    #[test]
    fn test_peek_returns_highest_priority() {
        let mut q = PriorityQueue::new();
        q.push(make_item("low", Priority::Low));
        q.push(make_item("high", Priority::High));
        let peeked = q.peek().unwrap();
        assert_eq!(peeked.id, "high");
        assert_eq!(q.len(), 2); // peek does not remove
    }

    #[test]
    fn test_drain_by_priority() {
        let mut q = PriorityQueue::new();
        q.push(make_item("c1", Priority::Critical));
        q.push(make_item("c2", Priority::Critical));
        q.push(make_item("m1", Priority::Medium));

        let drained = q.drain_by_priority(&Priority::Critical);
        assert_eq!(drained.len(), 2);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_priority_item_fields() {
        let item = make_item("test", Priority::High);
        assert_eq!(item.id, "test");
        assert_eq!(item.priority, Priority::High);
        assert!(!item.payload.is_empty());
        assert!(item.enqueued_at > 0);
    }

    #[test]
    fn test_multiple_same_priority_fifo() {
        let mut q = PriorityQueue::new();
        q.push(make_item("first", Priority::High));
        q.push(make_item("second", Priority::High));
        assert_eq!(q.pop().unwrap().id, "first");
        assert_eq!(q.pop().unwrap().id, "second");
    }
}

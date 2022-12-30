pub mod queue;
pub mod rr;

#[cfg(test)]
mod tests {
    use crate::queue::{Kind, Queue};
    use crate::rr::RR;

    #[test]
    fn random_replacement() {
        let mut rr_cache = RR::new(3);
        assert_eq!(rr_cache.get(1), None);
        assert_eq!(rr_cache.set(1, "one"), true);
        assert_eq!(rr_cache.get(1), Some(&"one"));

        assert_eq!(rr_cache.set(2, "two"), true);
        assert_eq!(rr_cache.set(3, "three"), true);
        assert_eq!(rr_cache.set(4, "four"), true);
        assert!(rr_cache.get(2).is_some());
    }

    #[test]
    fn queue_fifo() {
        let mut fifo_cache: Queue<i32, &str> = Queue::new(3, Kind::FIFO);
        assert_eq!(fifo_cache.get(1), None);
        assert_eq!(fifo_cache.set(1, "one"), true);
        assert_eq!(fifo_cache.get(1), Some(&"one"));

        assert_eq!(fifo_cache.set(2, "two"), true);
        assert_eq!(fifo_cache.set(3, "three"), true);
        assert_eq!(fifo_cache.set(4, "four"), true);
        assert!(fifo_cache.get(2).is_some());
    }

    #[test]
    fn queue_lifo() {
        let mut lifo_cache: Queue<i32, &str> = Queue::new(3, Kind::LIFO);
        assert_eq!(lifo_cache.get(1), None);
        assert_eq!(lifo_cache.set(1, "one"), true);
        assert_eq!(lifo_cache.get(1), Some(&"one"));

        assert_eq!(lifo_cache.set(2, "two"), true);
        assert_eq!(lifo_cache.set(3, "three"), true);
        assert_eq!(lifo_cache.set(4, "four"), true);
        assert!(lifo_cache.get(2).is_some());
    }
}

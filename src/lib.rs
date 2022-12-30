pub mod fifo;
pub mod rr;

#[cfg(test)]
mod tests {
    use crate::fifo::Fifo;
    use crate::rr::RR;

    #[test]
    fn rr() {
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
    fn fifo() {
        let mut fifo_cache: Fifo<i32, &str> = Fifo::new(3);
        assert_eq!(fifo_cache.get(1), None);
        assert_eq!(fifo_cache.set(1, "one"), true);
        assert_eq!(fifo_cache.get(1), Some(&"one"));

        assert_eq!(fifo_cache.set(2, "two"), true);
        assert_eq!(fifo_cache.set(3, "three"), true);
        assert_eq!(fifo_cache.set(4, "four"), true);
        assert!(fifo_cache.get(2).is_some());
    }
}

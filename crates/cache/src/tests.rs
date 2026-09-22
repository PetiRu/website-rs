#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn expired_entries_are_not_returned() {
        let cache = MemoryCache::new(4);
        cache.put("key".into(), b"value".to_vec(), Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(3));
        assert_eq!(cache.get("key"), None);
    }

    #[test]
    fn capacity_is_bounded() {
        let cache = MemoryCache::new(1);
        cache.put("one".into(), vec![1], Duration::from_secs(60));
        cache.put("two".into(), vec![2], Duration::from_secs(60));
        assert!(cache.get("one").is_none() || cache.get("two").is_none());
    }
}

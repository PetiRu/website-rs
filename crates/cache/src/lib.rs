//! A deliberately simple cache contract and bounded in-memory implementation.
use std::{collections::HashMap, sync::{Arc, RwLock}, time::{Duration, Instant}};

pub trait Cache: Send + Sync { fn get(&self, key: &str) -> Option<Vec<u8>>; fn put(&self, key: String, value: Vec<u8>, ttl: Duration); fn remove(&self, key: &str); }
struct Entry { value: Vec<u8>, expires: Instant }
#[derive(Clone)]
pub struct MemoryCache { inner: Arc<RwLock<HashMap<String, Entry>>>, capacity: usize }
impl MemoryCache { pub fn new(capacity: usize) -> Self { Self { inner: Arc::new(RwLock::new(HashMap::new())), capacity } }
    pub fn len(&self) -> usize { self.inner.read().map(|map| map.len()).unwrap_or(0) }
}
impl Cache for MemoryCache {
    fn get(&self, key: &str) -> Option<Vec<u8>> { let mut map = self.inner.write().ok()?; let e = map.get(key)?; if e.expires <= Instant::now() { map.remove(key); None } else { Some(e.value.clone()) } }
    fn put(&self, key: String, value: Vec<u8>, ttl: Duration) { if let Ok(mut map) = self.inner.write() { if self.capacity == 0 { return; } if map.len() >= self.capacity && !map.contains_key(&key) { if let Some(k) = map.keys().next().cloned() { map.remove(&k); } } map.insert(key, Entry { value, expires: Instant::now() + ttl }); } }
    fn remove(&self, key: &str) { if let Ok(mut map) = self.inner.write() { map.remove(key); } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn expired_entries_are_not_returned() { let cache = MemoryCache::new(4); cache.put("key".into(), b"value".to_vec(), Duration::from_millis(1)); std::thread::sleep(Duration::from_millis(3)); assert_eq!(cache.get("key"), None); }
    #[test]
    fn capacity_is_bounded() { let cache = MemoryCache::new(1); cache.put("one".into(), vec![1], Duration::from_secs(60)); cache.put("two".into(), vec![2], Duration::from_secs(60)); assert!(cache.len() <= 1); }
}

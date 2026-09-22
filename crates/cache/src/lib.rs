//! A deliberately simple cache contract and bounded in-memory implementation.
use std::{collections::HashMap, sync::{Arc, RwLock}, time::{Duration, Instant}};

pub trait Cache: Send + Sync { fn get(&self, key: &str) -> Option<Vec<u8>>; fn put(&self, key: String, value: Vec<u8>, ttl: Duration); fn remove(&self, key: &str); }
struct Entry { value: Vec<u8>, expires: Instant }
#[derive(Clone)]
pub struct MemoryCache { inner: Arc<RwLock<HashMap<String, Entry>>>, capacity: usize }
impl MemoryCache { pub fn new(capacity: usize) -> Self { Self { inner: Arc::new(RwLock::new(HashMap::new())), capacity } } }
impl Cache for MemoryCache {
    fn get(&self, key: &str) -> Option<Vec<u8>> { let mut map = self.inner.write().ok()?; let e = map.get(key)?; if e.expires <= Instant::now() { map.remove(key); None } else { Some(e.value.clone()) } }
    fn put(&self, key: String, value: Vec<u8>, ttl: Duration) { if let Ok(mut map) = self.inner.write() { if map.len() >= self.capacity && !map.contains_key(&key) { if let Some(k) = map.keys().next().cloned() { map.remove(&k); } } map.insert(key, Entry { value, expires: Instant::now() + ttl }); } }
    fn remove(&self, key: &str) { if let Ok(mut map) = self.inner.write() { map.remove(key); } }
}

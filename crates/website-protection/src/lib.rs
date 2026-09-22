//! Safe-by-default HTTP protection helpers.
use std::{collections::HashMap, sync::Mutex, time::{Duration, Instant}};

pub fn security_headers() -> [(&'static str, &'static str); 5] { [
    ("X-Content-Type-Options", "nosniff"), ("X-Frame-Options", "DENY"),
    ("Referrer-Policy", "strict-origin-when-cross-origin"), ("Content-Security-Policy", "default-src 'self'"),
    ("Strict-Transport-Security", "max-age=31536000; includeSubDomains"),
] }

pub fn origin_allowed(origin: &str, allowed: &[&str]) -> bool { allowed.iter().any(|item| *item == origin) }

pub struct RateLimiter { state: Mutex<HashMap<String, (u32, Instant)>>, max: u32, window: Duration }
impl RateLimiter {
    pub fn new(max: u32, window: Duration) -> Self { Self { state: Mutex::new(HashMap::new()), max, window } }
    pub fn allow(&self, identity: &str) -> bool { let Ok(mut state) = self.state.lock() else { return false }; let now = Instant::now(); let entry = state.entry(identity.to_owned()).or_insert((0, now)); if now.duration_since(entry.1) >= self.window { *entry = (0, now); } if entry.0 >= self.max { false } else { entry.0 += 1; true } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn headers_include_core_defenses() { let headers = security_headers(); assert!(headers.contains(&("X-Frame-Options", "DENY"))); assert!(headers.contains(&("Content-Security-Policy", "default-src 'self'"))); }
    #[test]
    fn origins_are_exact_matches() { assert!(origin_allowed("https://example.test", &["https://example.test"])); assert!(!origin_allowed("https://evil.example", &["https://example.test"])); }
    #[test]
    fn limiter_rejects_after_quota() { let limiter = RateLimiter::new(2, Duration::from_secs(60)); assert!(limiter.allow("client")); assert!(limiter.allow("client")); assert!(!limiter.allow("client")); assert!(limiter.allow("other")); }
}

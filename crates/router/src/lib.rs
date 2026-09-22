//! Minimal deterministic route table. Integrate it with an HTTP framework.
use website_cache::Cache;

pub struct Router<C> { routes: Vec<(String, String)>, cache: C }

impl<C: Cache> Router<C> {
    pub fn new(cache: C) -> Self { Self { routes: Vec::new(), cache } }

    /// Adds an exact path route. Earlier registrations take precedence.
    pub fn route(mut self, path: impl Into<String>, handler_id: impl Into<String>) -> Self {
        self.routes.push((path.into(), handler_id.into()));
        self
    }

    pub fn resolve(&self, path: &str) -> Option<&str> {
        self.routes.iter().find(|(route, _)| route == path).map(|(_, handler)| handler.as_str())
    }

    /// Produces a stable namespace for response caching.
    pub fn cache_key(&self, method: &str, path: &str) -> String {
        format!("{}:{}", method.to_ascii_uppercase(), path)
    }

    pub fn cache(&self) -> &C { &self.cache }
}

#[cfg(test)]
mod tests {
    use super::*;
    use website_cache::MemoryCache;
    #[test]
    fn resolves_exact_paths_and_builds_keys() {
        let router = Router::new(MemoryCache::new(4)).route("/health", "health");
        assert_eq!(router.resolve("/health"), Some("health"));
        assert_eq!(router.resolve("/health/"), None);
        assert_eq!(router.cache_key("get", "/health"), "GET:/health");
    }
}

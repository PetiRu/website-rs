//! Minimal route table. Integrate it with axum, actix-web, or another server.
use website_cache::Cache;

pub struct Router<C> { routes: Vec<(String, String)>, cache: C }
impl<C: Cache> Router<C> {
    pub fn new(cache: C) -> Self { Self { routes: Vec::new(), cache } }
    pub fn route(mut self, path: impl Into<String>, handler_id: impl Into<String>) -> Self { self.routes.push((path.into(), handler_id.into())); self }
    pub fn resolve(&self, path: &str) -> Option<&str> { self.routes.iter().find(|(p, _)| p == path).map(|(_, h)| h.as_str()) }
    pub fn cache(&self) -> &C { &self.cache }
}

//! Framework-neutral request and response building blocks.
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("request body exceeds the {limit} byte limit")]
    BodyTooLarge { limit: usize },
}

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: impl Into<String>, path: impl Into<String>, body: Vec<u8>) -> Self {
        Self { method: method.into(), path: path.into(), headers: BTreeMap::new(), body }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn enforce_body_limit(&self, limit: usize) -> Result<(), Error> {
        if self.body.len() > limit { Err(Error::BodyTooLarge { limit }) } else { Ok(()) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn text(status: u16, body: impl Into<Vec<u8>>) -> Self {
        Self { status, headers: BTreeMap::new(), body: body.into() }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn request_limit_is_inclusive() {
        let request = Request::new("POST", "/", vec![1, 2]);
        assert!(request.enforce_body_limit(2).is_ok());
        assert_eq!(request.enforce_body_limit(1), Err(Error::BodyTooLarge { limit: 1 }));
    }
    #[test]
    fn builders_add_headers() {
        let request = Request::new("GET", "/", vec![]).header("Accept", "application/json");
        let response = Response::text(200, "ok").header("Content-Type", "text/plain");
        assert_eq!(request.headers["Accept"], "application/json");
        assert_eq!(response.headers["Content-Type"], "text/plain");
    }
}

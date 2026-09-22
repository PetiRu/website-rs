//! Framework-neutral request and response building blocks.
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error { #[error("request body exceeds the {limit} byte limit")] BodyTooLarge { limit: usize } }

#[derive(Debug, Clone, Default)]
pub struct Request { pub method: String, pub path: String, pub headers: BTreeMap<String, String>, pub body: Vec<u8> }
impl Request {
    pub fn new(method: impl Into<String>, path: impl Into<String>, body: Vec<u8>) -> Self { Self { method: method.into(), path: path.into(), headers: BTreeMap::new(), body } }
    pub fn enforce_body_limit(&self, limit: usize) -> Result<(), Error> { if self.body.len() > limit { Err(Error::BodyTooLarge { limit }) } else { Ok(()) } }
}
#[derive(Debug, Clone)]
pub struct Response { pub status: u16, pub headers: BTreeMap<String, String>, pub body: Vec<u8> }
impl Response { pub fn text(status: u16, body: impl Into<Vec<u8>>) -> Self { Self { status, headers: BTreeMap::new(), body: body.into() } } }

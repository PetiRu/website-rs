//! Framework-neutral human-verification primitives.
//!
//! This is intentionally not a full visual CAPTCHA renderer. Applications can
//! use the challenge store directly or implement the `ChallengeStore` trait
//! for Redis, a database, or an external provider.
use rand::{distributions::Alphanumeric, Rng};
use std::{collections::HashMap, sync::Mutex, time::{Duration, Instant}};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("challenge was not found or has expired")]
    InvalidChallenge,
    #[error("answer was incorrect")]
    IncorrectAnswer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Challenge { pub token: String, pub prompt: String, pub expires_in: Duration }

pub trait ChallengeStore: Send + Sync {
    fn issue(&self, ttl: Duration) -> Challenge;
    fn verify(&self, token: &str, answer: &str) -> Result<(), Error>;
}

struct Pending { answer: String, expires: Instant }

/// A small single-process store for development and low-volume services.
/// Use a shared store for multiple application instances.
pub struct MemoryCaptcha { pending: Mutex<HashMap<String, Pending>> }

impl Default for MemoryCaptcha { fn default() -> Self { Self::new() } }
impl MemoryCaptcha {
    pub fn new() -> Self { Self { pending: Mutex::new(HashMap::new()) } }
    pub fn issue_math(&self, ttl: Duration) -> (Challenge, String) {
        let mut rng = rand::thread_rng();
        let left: u8 = rng.gen_range(2..20);
        let right: u8 = rng.gen_range(2..20);
        let answer = (left as u16 + right as u16).to_string();
        let challenge = self.issue_with_answer(format!("What is {left} + {right}?"), answer.clone(), ttl);
        (challenge, answer)
    }
    fn issue_with_answer(&self, prompt: String, answer: String, ttl: Duration) -> Challenge {
        let token: String = rand::thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
        let challenge = Challenge { token: token.clone(), prompt, expires_in: ttl };
        if let Ok(mut pending) = self.pending.lock() { pending.insert(token, Pending { answer, expires: Instant::now() + ttl }); }
        challenge
    }
}

impl ChallengeStore for MemoryCaptcha {
    fn issue(&self, ttl: Duration) -> Challenge { self.issue_math(ttl).0 }
    fn verify(&self, token: &str, answer: &str) -> Result<(), Error> {
        let Ok(mut pending) = self.pending.lock() else { return Err(Error::InvalidChallenge) };
        let Some(challenge) = pending.remove(token) else { return Err(Error::InvalidChallenge) };
        if challenge.expires <= Instant::now() { return Err(Error::InvalidChallenge); }
        if challenge.answer == answer.trim() { Ok(()) } else { Err(Error::IncorrectAnswer) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn issued_math_challenge_can_be_verified_once() {
        let store = MemoryCaptcha::new();
        let (challenge, answer) = store.issue_math(Duration::from_secs(30));
        assert!(store.verify(&challenge.token, &answer).is_ok());
        assert_eq!(store.verify(&challenge.token, &answer), Err(Error::InvalidChallenge));
    }
    #[test]
    fn wrong_answer_is_rejected() {
        let store = MemoryCaptcha::new();
        let (challenge, _) = store.issue_math(Duration::from_secs(30));
        assert_eq!(store.verify(&challenge.token, "wrong"), Err(Error::IncorrectAnswer));
    }
}

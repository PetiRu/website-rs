//! Provider-neutral abuse response primitives.
//!
//! This crate intentionally does not claim to identify "AI people" or ban an
//! IP as an identity. Signals are probabilistic and shared networks can contain
//! legitimate users. Use short-lived, graduated responses and retain an appeal
//! path. Infrastructure-level DDoS controls still belong at the provider edge.
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Response { Allow, Challenge, Throttle, TemporaryQuarantine }

#[derive(Debug, Clone, Copy, Default)]
pub struct Signals {
    pub requests_per_second: u32,
    pub concurrent_connections: u32,
    pub failed_challenges: u32,
    pub malformed_requests: u32,
    pub synchronized_sources: u32,
    pub authenticated_account: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Policy {
    pub challenge_score: u32,
    pub throttle_score: u32,
    pub quarantine_score: u32,
    pub max_quarantine: Duration,
}

impl Default for Policy {
    fn default() -> Self { Self { challenge_score: 20, throttle_score: 45, quarantine_score: 75, max_quarantine: Duration::from_secs(900) } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision { pub response: Response, pub score: u32, pub expires_in: Option<Duration> }

/// Scores behavior, not identity. Keep raw identifiers outside this crate and
/// use rotating keyed pseudonyms if correlation is operationally necessary.
pub fn evaluate(signals: Signals, policy: Policy) -> Decision {
    let score = signals.requests_per_second.min(40)
        + signals.concurrent_connections.min(20)
        + signals.failed_challenges.min(20)
        + signals.malformed_requests.min(15)
        + signals.synchronized_sources.min(20)
        - if signals.authenticated_account { 10 } else { 0 };
    let score = score.max(0);
    if score >= policy.quarantine_score {
        Decision { response: Response::TemporaryQuarantine, score, expires_in: Some(policy.max_quarantine) }
    } else if score >= policy.throttle_score {
        Decision { response: Response::Throttle, score, expires_in: Some(Duration::from_secs(120)) }
    } else if score >= policy.challenge_score {
        Decision { response: Response::Challenge, score, expires_in: Some(Duration::from_secs(300)) }
    } else {
        Decision { response: Response::Allow, score, expires_in: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_authenticated_traffic_is_allowed() {
        let decision = evaluate(Signals { authenticated_account: true, ..Signals::default() }, Policy::default());
        assert_eq!(decision.response, Response::Allow);
    }
    #[test]
    fn coordinated_abuse_gets_temporary_response() {
        let decision = evaluate(Signals { requests_per_second: 40, concurrent_connections: 20, failed_challenges: 20, malformed_requests: 15, synchronized_sources: 20, ..Signals::default() }, Policy::default());
        assert_eq!(decision.response, Response::TemporaryQuarantine);
        assert!(decision.expires_in.unwrap() <= Duration::from_secs(900));
    }
}

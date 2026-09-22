//! Stateless, authenticated CSRF token primitives.
use aes_gcm::{aead::{Aead, KeyInit, OsRng, rand_core::RngCore}, Aes256Gcm, Nonce};
use thiserror::Error;
use zeroize::Zeroizing;

const NONCE_LEN: usize = 12;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("token is malformed or authentication failed")]
    Invalid,
}

#[derive(Clone)]
pub struct Secret(Zeroizing<[u8; 32]>);
impl Secret { pub fn from_bytes(bytes: [u8; 32]) -> Self { Self(Zeroizing::new(bytes)) } }

/// Creates opaque tokens bound to a session or form context through AAD.
pub struct TokenManager { cipher: Aes256Gcm }
impl TokenManager {
    pub fn new(secret: Secret) -> Self { Self { cipher: Aes256Gcm::new_from_slice(&*secret.0).expect("32-byte key") } }

    pub fn issue(&self, subject: &[u8]) -> Result<Vec<u8>, Error> {
        let mut nonce = [0u8; NONCE_LEN]; OsRng.fill_bytes(&mut nonce);
        let value = self.cipher.encrypt(Nonce::from_slice(&nonce), aes_gcm::aead::Payload { msg: b"csrf:v1", aad: subject }).map_err(|_| Error::Invalid)?;
        let mut token = nonce.to_vec(); token.extend(value); Ok(token)
    }

    pub fn verify(&self, token: &[u8], subject: &[u8]) -> Result<(), Error> {
        if token.len() <= NONCE_LEN { return Err(Error::Invalid); }
        self.cipher.decrypt(Nonce::from_slice(&token[..NONCE_LEN]), aes_gcm::aead::Payload { msg: &token[NONCE_LEN..], aad: subject }).map_err(|_| Error::Invalid).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_is_bound_to_subject() {
        let manager = TokenManager::new(Secret::from_bytes([9; 32]));
        let token = manager.issue(b"session:42").unwrap();
        assert!(manager.verify(&token, b"session:42").is_ok());
        assert_eq!(manager.verify(&token, b"session:other"), Err(Error::Invalid));
    }
    #[test]
    fn malformed_tokens_fail_closed() {
        let manager = TokenManager::new(Secret::from_bytes([8; 32]));
        assert_eq!(manager.verify(&[0; NONCE_LEN], b"x"), Err(Error::Invalid));
    }
}

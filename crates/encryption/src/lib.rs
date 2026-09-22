//! Small, opinionated AES-256-GCM wrapper.
use aes_gcm::{aead::{Aead, KeyInit, OsRng, rand_core::RngCore}, Aes256Gcm, Nonce};
use thiserror::Error;
use zeroize::Zeroizing;

const NONCE_LEN: usize = 12;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ciphertext is too short")]
    TooShort,
    #[error("decryption failed or authentication tag was invalid")]
    Authentication,
}

/// A 256-bit key. Keep it in a secret manager and rotate it deliberately.
#[derive(Clone)]
pub struct Key(Zeroizing<[u8; 32]>);

impl Key {
    pub fn from_bytes(bytes: [u8; 32]) -> Self { Self(Zeroizing::new(bytes)) }
}

/// AES-256-GCM sealer. Output is nonce || ciphertext || authentication tag.
pub struct Sealer { cipher: Aes256Gcm }

impl Sealer {
    pub fn new(key: Key) -> Self { Self { cipher: Aes256Gcm::new_from_slice(&*key.0).expect("32-byte key") } }

    pub fn seal(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce);
        let encrypted = self.cipher.encrypt(Nonce::from_slice(&nonce), aes_gcm::aead::Payload { msg: plaintext, aad: associated_data }).map_err(|_| Error::Authentication)?;
        let mut output = nonce.to_vec();
        output.extend(encrypted);
        Ok(output)
    }

    pub fn open(&self, token: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, Error> {
        if token.len() < NONCE_LEN { return Err(Error::TooShort); }
        self.cipher.decrypt(Nonce::from_slice(&token[..NONCE_LEN]), aes_gcm::aead::Payload { msg: &token[NONCE_LEN..], aad: associated_data }).map_err(|_| Error::Authentication)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_and_aad() {
        let s = Sealer::new(Key::from_bytes([1; 32]));
        let t = s.seal(b"secret", b"profile").unwrap();
        assert_eq!(s.open(&t, b"profile").unwrap(), b"secret");
        assert!(s.open(&t, b"other").is_err());
    }
}

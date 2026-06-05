//! AES-256-GCM symmetric encryption.
//!
//! Wire format: [ 12-byte random nonce | ciphertext | 16-byte GCM auth tag ]

use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Key, Nonce};
use rand::RngCore;
use sha2::{Digest, Sha256};
use crate::error::CoreError;

const NONCE_LEN: usize = 12;

#[derive(Clone)]
pub struct SessionKey { cipher: Aes256Gcm }

impl SessionKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(bytes);
        Self { cipher: Aes256Gcm::new(key) }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CoreError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
        let nonce      = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher.encrypt(nonce, plaintext).map_err(CoreError::from)?;
        let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CoreError> {
        if data.len() < NONCE_LEN + 16 {
            return Err(CoreError::Crypto("Ciphertext too short".into()));
        }
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);
        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|_| CoreError::Crypto("Decryption failed — wrong key or data tampered".into()))
    }
}

pub fn sha256_hex(data: &[u8]) -> String { format!("{:x}", Sha256::digest(data)) }

#[cfg(test)]
mod tests {
    use super::*;
    fn key() -> SessionKey { SessionKey::from_bytes(&[0x42u8; 32]) }

    #[test] fn roundtrip() {
        let k = key(); let pt = b"borderless input event";
        assert_eq!(k.decrypt(&k.encrypt(pt).unwrap()).unwrap(), pt);
    }
    #[test] fn nonce_is_random() {
        let k = key();
        assert_ne!(&k.encrypt(b"x").unwrap()[..12], &k.encrypt(b"x").unwrap()[..12]);
    }
    #[test] fn wrong_key_fails() {
        let ct = SessionKey::from_bytes(&[0xABu8; 32]).encrypt(b"s").unwrap();
        assert!(SessionKey::from_bytes(&[0xCDu8; 32]).decrypt(&ct).is_err());
    }
    #[test] fn tamper_detection() {
        let k = key(); let mut ct = k.encrypt(b"important").unwrap();
        ct[20] ^= 0xFF; assert!(k.decrypt(&ct).is_err());
    }
}

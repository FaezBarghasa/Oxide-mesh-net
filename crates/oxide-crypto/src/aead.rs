//! AEAD encryption and decryption

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use crate::{AeadKey, AeadNonce, error::{CryptoError, Result}};

/// Encrypt data using ChaCha20-Poly1305
pub fn encrypt(key: &AeadKey, nonce: &AeadNonce, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    cipher.encrypt(nonce, chacha20poly1305::aead::Payload { msg: plaintext, aad })
        .map_err(|e| CryptoError::Encryption(e.to_string()))
}

/// Decrypt data using ChaCha20-Poly1305
pub fn decrypt(key: &AeadKey, nonce: &AeadNonce, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    cipher.decrypt(nonce, chacha20poly1305::aead::Payload { msg: ciphertext, aad })
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}

/// Encrypt in-place using ChaCha20-Poly1305 (for zero-copy operations)
pub fn encrypt_in_place(key: &AeadKey, nonce: &AeadNonce, buffer: &mut [u8], aad: &[u8]) -> Result<usize> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    let tag = cipher.encrypt_in_place(nonce, aad, buffer)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;
    Ok(tag.len())
}

/// Decrypt in-place using ChaCha20-Poly1305
pub fn decrypt_in_place(key: &AeadKey, nonce: &AeadNonce, buffer: &mut [u8], aad: &[u8]) -> Result<()> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    cipher.decrypt_in_place(nonce, aad, buffer)
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}

/// Generate a random nonce
pub fn random_nonce() -> AeadNonce {
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    AeadNonce::from_bytes(nonce.as_slice())
}
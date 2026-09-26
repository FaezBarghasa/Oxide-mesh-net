//! AEAD encryption and decryption

use chacha20poly1305::{
    aead::{Aead, AeadInPlace, KeyInit},
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
pub fn encrypt_in_place(key: &AeadKey, nonce: &AeadNonce, buffer: &mut [u8], aad: &[u8]) -> Result<[u8; 16]> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    let tag = cipher.encrypt_in_place_detached(nonce, aad, buffer)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;
    Ok(tag.into())
}

/// Decrypt in-place using ChaCha20-Poly1305
pub fn decrypt_in_place(key: &AeadKey, nonce: &AeadNonce, buffer: &mut [u8], tag: &[u8; 16], aad: &[u8]) -> Result<()> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    let nonce = Nonce::from_slice(nonce.as_bytes());
    let tag = chacha20poly1305::Tag::from_slice(tag);
    cipher.decrypt_in_place_detached(nonce, aad, buffer, tag)
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}

/// Generate a random nonce
pub fn random_nonce() -> AeadNonce {
    let mut bytes = [0u8; 12];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut bytes);
    AeadNonce(bytes)
}
//! Cryptographic error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Key generation failed: {0}")]
    KeyGeneration(String),

    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),

    #[error("Encryption failed: {0}")]
    Encryption(String),

    #[error("Decryption failed: {0}")]
    Decryption(String),

    #[error("Signature failed: {0}")]
    Signature(String),

    #[error("Verification failed: {0}")]
    Verification(String),

    #[error("Key agreement failed: {0}")]
    KeyAgreement(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Algorithm not supported: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Hardware token error: {0}")]
    HardwareToken(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, CryptoError>;

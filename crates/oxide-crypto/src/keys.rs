//! Cryptographic key types and identity management

use std::fmt;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519StaticSecret};
use ed25519_dalek::{
    SigningKey as Ed25519SigningKey, VerifyingKey as Ed25519VerifyingKey,
    Signature as Ed25519Signature, Signer, Verifier,
};
use blake3;

/// Long-term device identity key pair (Ed25519 for signing)
#[derive(Clone)]
pub struct DeviceIdentityKey {
    pub(crate) signing_key: Ed25519SigningKey,
}

impl Zeroize for DeviceIdentityKey {
    fn zeroize(&mut self) {
        self.signing_key = Ed25519SigningKey::from_bytes(&[0u8; 32]);
    }
}

impl Drop for DeviceIdentityKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl DeviceIdentityKey {
    pub fn generate() -> Self {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = Ed25519SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, crate::error::CryptoError> {
        let signing_key = Ed25519SigningKey::from_bytes(bytes);
        Ok(Self { signing_key })
    }

    pub fn public_key(&self) -> DeviceIdentityPublicKey {
        DeviceIdentityPublicKey {
            verifying_key: self.signing_key.verifying_key(),
        }
    }

    pub fn sign(&self, msg: &[u8]) -> DeviceSignature {
        let sig = self.signing_key.sign(msg);
        DeviceSignature(sig)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    pub fn signing_key(&self) -> &Ed25519SigningKey {
        &self.signing_key
    }
}

impl fmt::Debug for DeviceIdentityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceIdentityKey")
            .field("signing_key", &"[REDACTED]")
            .finish()
    }
}

impl PartialEq for DeviceIdentityKey {
    fn eq(&self, other: &Self) -> bool {
        self.signing_key.to_bytes() == other.signing_key.to_bytes()
    }
}

impl Eq for DeviceIdentityKey {}

/// Public component of device identity key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceIdentityPublicKey {
    verifying_key: Ed25519VerifyingKey,
}

impl DeviceIdentityPublicKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, crate::error::CryptoError> {
        let verifying_key = Ed25519VerifyingKey::from_bytes(bytes)
            .map_err(|e| crate::error::CryptoError::InvalidKeyFormat(e.to_string()))?;
        Ok(Self { verifying_key })
    }

    pub fn verify(&self, msg: &[u8], sig: &DeviceSignature) -> Result<(), crate::error::CryptoError> {
        self.verifying_key
            .verify(msg, &sig.0)
            .map_err(|e| crate::error::CryptoError::Verification(e.to_string()))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.verifying_key.as_bytes()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        *self.verifying_key.as_bytes()
    }

    pub fn fingerprint(&self) -> KeyFingerprint {
        let hash = blake3::hash(self.verifying_key.as_bytes());
        KeyFingerprint(*hash.as_bytes())
    }
}

impl fmt::Display for DeviceIdentityPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.verifying_key.as_bytes()).into_string())
    }
}

impl std::str::FromStr for DeviceIdentityPublicKey {
    type Err = crate::error::CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec().map_err(|e| crate::error::CryptoError::InvalidKeyFormat(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(crate::error::CryptoError::InvalidKeyFormat("Invalid key length".into()));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Self::from_bytes(&arr)
    }
}

/// Device signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSignature(Ed25519Signature);

impl DeviceSignature {
    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8; 64]) -> Result<Self, crate::error::CryptoError> {
        let sig = Ed25519Signature::from_bytes(bytes);
        Ok(Self(sig))
    }
}

/// Key fingerprint (Blake3 hash of public key)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyFingerprint(pub [u8; 32]);

impl KeyFingerprint {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn short(&self) -> String {
        hex::encode(&self.0[..8])
    }
}

impl fmt::Display for KeyFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Ephemeral session key pair (X25519 for key agreement)
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SessionKeyPair {
    static_secret: X25519StaticSecret,
    public_key: X25519PublicKey,
}

pub type SessionKey = SessionKeyPair;

impl SessionKeyPair {
    pub fn generate() -> Self {
        let mut csprng = rand::rngs::OsRng;
        let static_secret = X25519StaticSecret::random_from_rng(&mut csprng);
        let public_key = X25519PublicKey::from(&static_secret);
        Self { static_secret, public_key }
    }

    pub fn from_secret(secret: X25519StaticSecret) -> Self {
        let public_key = X25519PublicKey::from(&secret);
        Self { static_secret: secret, public_key }
    }

    pub fn public_key(&self) -> SessionPublicKey {
        SessionPublicKey(self.public_key)
    }

    pub fn diffie_hellman(&self, peer_public: &SessionPublicKey) -> SharedSecret {
        let shared = self.static_secret.diffie_hellman(&peer_public.0);
        SharedSecret::from_bytes(shared.as_bytes())
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.static_secret.to_bytes()
    }
}

impl fmt::Debug for SessionKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionKeyPair")
            .field("static_secret", &"[REDACTED]")
            .field("public_key", &self.public_key)
            .finish()
    }
}

/// Public component of session key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionPublicKey(pub X25519PublicKey);

impl SessionPublicKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, crate::error::CryptoError> {
        let pk = X25519PublicKey::from(*bytes);
        Ok(Self(pk))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        *self.0.as_bytes()
    }
}

impl fmt::Display for SessionPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0.as_bytes()).into_string())
    }
}

impl std::str::FromStr for SessionPublicKey {
    type Err = crate::error::CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec().map_err(|e| crate::error::CryptoError::InvalidKeyFormat(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(crate::error::CryptoError::InvalidKeyFormat("Invalid key length".into()));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Self::from_bytes(&arr)
    }
}

/// Shared secret from Diffie-Hellman
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SharedSecret(pub [u8; 32]);

impl SharedSecret {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Self(arr)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn derive_keys(&self, salt: &[u8], info: &[u8], output_len: usize) -> Vec<u8> {
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(salt), &self.0);
        let mut okm = vec![0u8; output_len];
        hk.expand(info, &mut okm).expect("HKDF expand should not fail");
        okm
    }

    pub fn derive_aead_key(&self, context: &[u8]) -> (AeadKey, AeadNonce) {
        let okm = self.derive_keys(b"oxide-mesh-aead", context, 44);
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];
        key.copy_from_slice(&okm[..32]);
        nonce.copy_from_slice(&okm[32..44]);
        (AeadKey(key), AeadNonce(nonce))
    }
}

impl fmt::Debug for SharedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedSecret").field("bytes", &"[REDACTED]").finish()
    }
}

/// AEAD encryption key (ChaCha20-Poly1305 or AES-GCM)
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct AeadKey(pub [u8; 32]);

impl AeadKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Self(arr)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for AeadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AeadKey").field("bytes", &"[REDACTED]").finish()
    }
}

/// AEAD nonce (12 bytes for ChaCha20-Poly1305 / AES-GCM)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AeadNonce(pub [u8; 12]);

impl AeadNonce {
    pub fn from_bytes(bytes: &[u8; 12]) -> Self {
        let mut arr = [0u8; 12];
        arr.copy_from_slice(bytes);
        Self(arr)
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }

    pub fn increment(&mut self) {
        for byte in self.0.iter_mut().rev() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
    }
}
//! Key agreement and session establishment (Noise-like handshake)

use crate::{
    keys::{
        DeviceIdentityKey, DeviceIdentityPublicKey, DeviceSignature,
        SessionKeyPair, SessionPublicKey, SharedSecret, AeadKey, AeadNonce,
    },
    aead::{encrypt, decrypt, random_nonce},
    error::{CryptoError, Result},
};
use blake3;
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Handshake pattern: NK (One-way authenticated, initiator knows responder's static key)
/// This is similar to Noise NK pattern used in WireGuard
pub const HANDSHAKE_NAME: &[u8] = b"Noise_NK_25519_ChaChaPoly_BLAKE3";

/// Handshake state machine
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct HandshakeState {
    // Symmetric state
    ck: [u8; 32],  // Chaining key
    h: [u8; 32],   // Hash
    k: Option<[u8; 32]>, // Encryption key (None until Split())
    n: u64,        // Nonce counter

    // Ephemeral keys
    e_priv: Option<SessionKeyPair>,
    e_pub: Option<SessionPublicKey>,

    // Static keys
    s_priv: Option<DeviceIdentityKey>,
    s_pub: Option<DeviceIdentityPublicKey>,

    // Peer static key (known for NK pattern)
    rs_pub: Option<DeviceIdentityPublicKey>,
}

impl HandshakeState {
    /// Initialize as initiator (NK pattern)
    pub fn initiator(
        our_static: DeviceIdentityKey,
        peer_static_pub: DeviceIdentityPublicKey,
    ) -> Self {
        let mut h = blake3::hash(HANDSHAKE_NAME).as_bytes().clone();
        let ck = h;

        // Mix our static public key into hash
        let our_static_pub = our_static.public_key();
        mix_hash(&mut h, our_static_pub.as_bytes());

        // Mix peer static public key into hash
        mix_hash(&mut h, peer_static_pub.as_bytes());

        // Generate ephemeral key
        let e_priv = SessionKeyPair::generate();
        let e_pub = e_priv.public_key();

        // Mix ephemeral public key into hash
        mix_hash(&mut h, e_pub.as_bytes());

        // DH(ephemeral, peer_static) -> ck, k
        let dh1 = e_priv.diffie_hellman(&SessionPublicKey::from_bytes(peer_static_pub.as_bytes()).unwrap());
        let (new_ck, k) = hkdf2(&ck, dh1.as_bytes());
        let ck = new_ck;

        let mut n = 0;

        // Encrypt our static public key
        let mut msg = our_static_pub.as_bytes().to_vec();
        let nonce = AeadNonce::from_bytes(&nonce_bytes(n));
        let aead_key = AeadKey(k);
        let ct = encrypt(&aead_key, &nonce, &msg, &h).unwrap();
        n += 1;

        mix_hash(&mut h, &ct);

        Self {
            ck,
            h,
            k: Some(k),
            n,
            e_priv: Some(e_priv),
            e_pub: Some(e_pub),
            s_priv: Some(our_static),
            s_pub: Some(our_static_pub),
            rs_pub: Some(peer_static_pub),
        }
    }

    /// Initialize as responder (NK pattern)
    pub fn responder(
        our_static: DeviceIdentityKey,
    ) -> Self {
        let mut h = blake3::hash(HANDSHAKE_NAME).as_bytes().clone();
        let ck = h;

        // Mix our static public key into hash
        let our_static_pub = our_static.public_key();
        mix_hash(&mut h, our_static_pub.as_bytes());

        Self {
            ck,
            h,
            k: None,
            n: 0,
            e_priv: None,
            e_pub: None,
            s_priv: Some(our_static),
            s_pub: Some(our_static_pub),
            rs_pub: None,
        }
    }

    /// Process initiator's first message (responder side)
    pub fn read_message_1(&mut self, msg: &[u8]) -> Result<()> {
        // Mix peer ephemeral public key
        if msg.len() < 32 {
            return Err(CryptoError::InvalidParameter("Message too short".into()));
        }
        let peer_e_pub = SessionPublicKey::from_bytes(msg[..32].try_into().unwrap())?;
        self.e_pub = Some(peer_e_pub);
        mix_hash(&mut self.h, peer_e_pub.as_bytes());

        // DH(our_static, peer_ephemeral) -> ck, k
        let our_static = self.s_priv.as_ref().unwrap();
        let dh1 = our_static.signing_key.diffie_hellman(&peer_e_pub.0); // This won't work directly
        // Need to use X25519 for DH
        // For now, use a placeholder

        // Decrypt peer static public key
        // This is complex - we need the full Noise implementation
        // For now, return error
        Err(CryptoError::UnsupportedAlgorithm("Full Noise NK not yet implemented".into()))
    }

    /// Complete handshake and derive transport keys
    pub fn split(self) -> Result<TransportKeys> {
        let k = self.k.ok_or(CryptoError::KeyDerivation("Handshake not complete".into()))?;
        let (tx_key, rx_key) = hkdf2(&k, b"");
        let tx_key = AeadKey::from_bytes(&tx_key);
        let rx_key = AeadKey::from_bytes(&rx_key);
        Ok(TransportKeys { tx_key, rx_key, tx_nonce: 0, rx_nonce: 0 })
    }

    /// Get handshake hash for channel binding
    pub fn handshake_hash(&self) -> [u8; 32] {
        self.h
    }
}

/// Transport keys derived from handshake
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct TransportKeys {
    pub tx_key: AeadKey,
    pub rx_key: AeadKey,
    pub tx_nonce: u64,
    pub rx_nonce: u64,
}

impl TransportKeys {
    pub fn encrypt(&mut self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let nonce = AeadNonce::from_bytes(&nonce_bytes(self.tx_nonce));
        self.tx_nonce += 1;
        encrypt(&self.tx_key, &nonce, plaintext, aad)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let nonce = AeadNonce::from_bytes(&nonce_bytes(self.rx_nonce));
        self.rx_nonce += 1;
        decrypt(&self.rx_key, &nonce, ciphertext, aad)
    }
}

/// Mix data into hash: h = HASH(h || data)
fn mix_hash(h: &mut [u8; 32], data: &[u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(h);
    hasher.update(data);
    *h = *hasher.finalize().as_bytes();
}

/// HKDF with 2 outputs: (ck', k) = HKDF(ck, input)
fn hkdf2(ck: &[u8; 32], input: &[u8]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha256>::new(Some(ck), input);
    let mut okm = [0u8; 64];
    hk.expand(b"", &mut okm).unwrap();
    let mut ck = [0u8; 32];
    let mut k = [0u8; 32];
    ck.copy_from_slice(&okm[..32]);
    k.copy_from_slice(&okm[32..64]);
    (ck, k)
}

/// Convert u64 to 12-byte nonce (little-endian + zeros)
fn nonce_bytes(n: u64) -> [u8; 12] {
    let mut bytes = [0u8; 12];
    bytes[..8].copy_from_slice(&n.to_le_bytes());
    bytes
}

/// Post-quantum hybrid key exchange placeholder
/// When PQC is available, this will combine X25519 + ML-KEM
pub mod pqc {
    use super::*;
    use zeroize::{Zeroize, ZeroizeOnDrop};

    /// Hybrid shared secret: X25519 || ML-KEM
    #[derive(Clone, Zeroize, ZeroizeOnDrop)]
    pub struct HybridSharedSecret {
        pub classical: SharedSecret,
        pub post_quantum: Vec<u8>, // ML-KEM shared secret
    }

    impl HybridSharedSecret {
        pub fn derive_keys(&self, salt: &[u8], info: &[u8], output_len: usize) -> Vec<u8> {
            // Combine classical and post-quantum secrets
            let mut combined = Vec::with_capacity(32 + self.post_quantum.len());
            combined.extend_from_slice(self.classical.as_bytes());
            combined.extend_from_slice(&self.post_quantum);

            let hk = Hkdf::<Sha256>::new(Some(salt), &combined);
            let mut okm = vec![0u8; output_len];
            hk.expand(info, &mut okm).unwrap();
            okm
        }
    }
}
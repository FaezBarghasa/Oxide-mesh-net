# handshake

## Classs

- [HandshakeState](HandshakeState.md) — Handshake state machine
- [HybridSharedSecret](HybridSharedSecret.md) — Hybrid shared secret: X25519 || ML-KEM
- [TransportKeys](TransportKeys.md) — Transport keys derived from handshake

## Functions

- [decrypt](decrypt.md)
- [decrypt](decrypt_1.md)
- [derive_keys](derive_keys.md)
- [derive_keys](derive_keys_1.md)
- [drop](drop.md)
- [drop](drop_1.md)
- [encrypt](encrypt.md)
- [encrypt](encrypt_1.md)
- [handshake_hash](handshake_hash.md) — Get handshake hash for channel binding
- [handshake_hash](handshake_hash_1.md) — Get handshake hash for channel binding
- [hkdf2](hkdf2.md) — HKDF with 2 outputs: (ck', k) = HKDF(ck, input)
- [initiator](initiator.md) — Initialize as initiator (NK pattern)
- [initiator](initiator_1.md) — Initialize as initiator (NK pattern)
- [mix_hash](mix_hash.md) — Mix data into hash: h = HASH(h || data)
- [nonce_bytes](nonce_bytes.md) — Convert u64 to 12-byte nonce (little-endian + zeros)
- [read_message_1](read_message_1.md) — Process initiator's first message (responder side)
- [read_message_1](read_message_1_1.md) — Process initiator's first message (responder side)
- [responder](responder.md) — Initialize as responder (NK pattern)
- [responder](responder_1.md) — Initialize as responder (NK pattern)
- [split](split.md) — Complete handshake and derive transport keys
- [split](split_1.md) — Complete handshake and derive transport keys
- [zeroize](zeroize.md)
- [zeroize](zeroize_1.md)

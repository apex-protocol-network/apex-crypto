# apex-crypto

Post-quantum cryptographic primitives for the APEX protocol.

## Primitives

| Primitive | Standard | Role |
|-----------|----------|------|
| CRYSTALS-Dilithium3 | NIST FIPS 204 | Primary transaction signatures |
| CRYSTALS-Kyber-768 | NIST FIPS 203 | Session key encapsulation |
| SPHINCS+-SHA2-256f | NIST FIPS 205 | Hash-based signature fallback |
| SHA3-256 | FIPS 202 | Address derivation, commitments |
| BLAKE3 | — | Merkle tree construction |

## Key Sizes (Dilithium3)

| Parameter | Size |
|-----------|------|
| Public key | 1,952 bytes |
| Secret key | 4,000 bytes |
| Signature | 3,293 bytes |

## Status

> **Pre-alpha.** Primitive selection follows NIST FIPS 203/204/205 (finalized 2024).

## Requirements

- Rust 1.75+
- C toolchain (for pqcrypto FFI bindings)

## Security

- Secret keys are zeroized on drop via the `zeroize` crate.
- No `unsafe` code.
- Hybrid mode (Dilithium3 + Ed25519) supported during the transition window.

## License

[Apache License 2.0](./LICENSE)

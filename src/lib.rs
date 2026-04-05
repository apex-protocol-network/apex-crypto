//! # apex-crypto
//!
//! Post-quantum cryptographic primitives for the APEX protocol.
//!
//! All secret key material is zeroized on drop.
//! No unsafe code.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod error;
pub mod keys;
pub mod signatures;
pub mod utils;

//! # xkcp-rs
//!
//! Safe wrappers to the [eXtended Keccak Code Package (XKCP)](https://github.com/XKCP/XKCP) library.

#![doc(html_root_url = "https://danipopes.github.io/xkcp-rs")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub extern crate xkcp_sys as ffi;

mod error;
mod keccak;

pub use error::{Error, Result};
pub use keccak::KeccakHash;

/// Function to evaluate the sponge function Keccak\[r, c] in a single call.
#[inline]
pub fn keccak_sponge(
    rate: u32,
    capacity: u32,
    suffix: u8,
    input: &[u8],
    output: &mut [u8],
) -> Result<()> {
    Error::from_int(unsafe {
        ffi::KeccakWidth1600_Sponge(
            rate,
            capacity,
            input.as_ptr(),
            input.len(),
            suffix,
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

/// Implementation of the SHAKE128 extendable output function (XOF) \[FIPS 202].
#[inline]
pub fn shake128(input: &[u8], output: &mut [u8]) -> Result<()> {
    keccak_sponge(1344, 256, 0x1F, input, output)
}

/// Implementation of the SHAKE256 extendable output function (XOF) \[FIPS 202].
#[inline]
pub fn shake256(input: &[u8], output: &mut [u8]) -> Result<()> {
    keccak_sponge(1088, 512, 0x1F, input, output)
}

/// Implementation of SHA3-224 \[FIPS 202].
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sha3_224(input: &[u8], output: &mut [u8; 28]) {
    let result = keccak_sponge(1152, 448, 0x06, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of SHA3-256 \[FIPS 202].
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sha3_256(input: &[u8], output: &mut [u8; 32]) {
    let result = keccak_sponge(1088, 512, 0x06, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of SHA3-384 \[FIPS 202].
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sha3_384(input: &[u8], output: &mut [u8; 48]) {
    let result = keccak_sponge(832, 768, 0x06, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of SHA3-512 \[FIPS 202].
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sha3_512(input: &[u8], output: &mut [u8; 64]) {
    let result = keccak_sponge(576, 1024, 0x06, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of Keccak-224.
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn keccak224(input: &[u8], output: &mut [u8; 28]) {
    let result = keccak_sponge(1152, 448, 0x01, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of Keccak-256.
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn keccak256(input: &[u8], output: &mut [u8; 32]) {
    let result = keccak_sponge(1088, 512, 0x01, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of Keccak-384.
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn keccak384(input: &[u8], output: &mut [u8; 48]) {
    let result = keccak_sponge(832, 768, 0x01, input, output);
    debug_assert_eq!(result, Ok(()));
}

/// Implementation of Keccak-512.
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn keccak512(input: &[u8], output: &mut [u8; 64]) {
    let result = keccak_sponge(576, 1024, 0x01, input, output);
    debug_assert_eq!(result, Ok(()));
}

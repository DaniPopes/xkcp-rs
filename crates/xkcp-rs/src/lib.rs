//! # xkcp-rs
//!
//! Safe wrappers to the [eXtended Keccak Code Package (XKCP)](https://github.com/XKCP/XKCP) library.

#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate xkcp_sys as ffi;

mod error;
mod keccak;

pub use error::{Error, Result};
pub use keccak::KeccakHash;

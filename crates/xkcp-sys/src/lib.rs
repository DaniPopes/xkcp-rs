//! # xkcp-sys
//!
//! Native bindings to the [eXtended Keccak Code Package (XKCP)](https://github.com/XKCP/XKCP) library.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(improper_ctypes)] // u128 used for some reason

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

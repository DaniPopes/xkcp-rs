//! # xkcp-sys
//!
//! Native bindings to the [eXtended Keccak Code Package (XKCP)](https://github.com/XKCP/XKCP) library.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#![cfg_attr(not(feature = "native"), no_std)]
mod helpers;
mod rpc;
pub mod spec;

#[cfg(feature = "native")]
pub mod service;
pub mod verifier;

const REVEAL_OUTPUT_AMOUNT: u64 = 546;

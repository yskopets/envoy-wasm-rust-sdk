//! Raw API bindings to the Envoy ABI
//!
//! This crate is entirely procedurally generated from the `*.witx` files that
//! describe the Envoy ABI.

#![no_std]

mod error;
mod types_generated;
mod envoy_host_generated;
pub use types_generated::*;
pub use envoy_host_generated::*;

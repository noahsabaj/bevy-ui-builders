//! Utility functions for UI builders

pub mod intrinsic;

// Gateway exports - external code MUST access through here!
// These exports are used by lib.rs to provide the public API.
#[allow(unused_imports)]
pub use intrinsic::*;
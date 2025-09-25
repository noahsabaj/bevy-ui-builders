//! Shared styling system for all UI builders
//!
//! Provides consistent colors, dimensions, and style variants
//! that can be used across all builders for a cohesive look.

mod button_styles;
pub mod colors;
pub mod dimensions;

// Re-export everything from submodules - GATEWAY PATTERN
// External code must ONLY access through this gateway!
// These exports are used by lib.rs to provide the public API.
#[allow(unused_imports)]
pub use button_styles::{ButtonStyle, ButtonSize};
#[allow(unused_imports)]
pub use colors::*;
#[allow(unused_imports)]
pub use dimensions::*;
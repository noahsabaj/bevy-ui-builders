//! Separator module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod types;

// Public exports only
pub use builder::{SeparatorBuilder, separator, separator_vertical};
pub use types::{Separator, SeparatorStyle, Orientation};
//! Label module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod types;

// Public exports only
pub use builder::{LabelBuilder, label};
pub use types::{Label, LabelStyle};
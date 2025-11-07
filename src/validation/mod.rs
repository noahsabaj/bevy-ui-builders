//! Universal input validation system
//!
//! Provides composable validation rules that work with any input component.

mod types;
mod systems;
mod plugin;

pub use types::{Validated, ValidationState};
pub use plugin::ValidationPlugin;

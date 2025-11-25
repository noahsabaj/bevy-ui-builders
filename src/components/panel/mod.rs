//! Panel module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod types;

// Public exports only
pub use builder::{PanelBuilder, panel};
pub use types::{Panel, PanelStyle};
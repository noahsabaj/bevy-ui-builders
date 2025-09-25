//! Custom UI relationships for Bevy 0.16
//!
//! This module provides bidirectional relationships for UI components
//! using Bevy 0.16's new relationship system.

mod types;
mod systems;
mod plugin;

// Gateway exports - ONLY exports, NO implementation!
pub use types::*;
pub use systems::*;
pub use plugin::UIRelationshipsPlugin;
//! Shared systems for all UI builders

pub mod cleanup;
pub mod hover;
pub mod interaction;

pub use cleanup::{despawn_entities, despawn_ui_entities};
pub use hover::HoverPlugin;
pub use interaction::handle_interactions;
//! Shared systems for all UI builders

pub mod cleanup;
pub mod hover;
pub mod interaction;

// Gateway exports - external code MUST access through here!
// These exports are used by lib.rs to provide the public API.
#[allow(unused_imports)]
pub use cleanup::{despawn_entities, despawn_ui_entities};
#[allow(unused_imports)]
pub use hover::HoverPlugin;
#[allow(unused_imports)]
pub use interaction::handle_interactions;
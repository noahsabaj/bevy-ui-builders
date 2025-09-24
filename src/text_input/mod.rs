//! Text input module - Gateway only

mod builder;
mod plugin;
mod systems;
mod types;

// Public exports only
pub use builder::{TextInputBuilder, TextInputBuilderWithMarker, TextInputBuilderWithTwoMarkers, text_input};
pub use plugin::TextInputPlugin;
pub use systems::*; // Export all systems for external customization if needed
pub use types::{
    TextInputFilter,
    InputFilter,
    InputTransform,
    ClearButtonTarget,
    TextInputFocus,
    FocusGroupId,
};
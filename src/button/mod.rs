//! Button module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod plugin;
mod systems;
mod types;

// Public exports only
pub use builder::{
    ButtonBuilder,
    ButtonBuilderWithMarker,
    primary_button,
    secondary_button,
    success_button,
    danger_button,
    ghost_button,
};
pub use plugin::ButtonPlugin;
pub use types::{
    StyledButton,
    SelectableButton,
    Selected,
    Active,
    ButtonSelectionColors,
    StateColorSet,
    SelectionChanged,
};

// Re-export styles for convenience
pub use crate::styles::{ButtonStyle, ButtonSize};
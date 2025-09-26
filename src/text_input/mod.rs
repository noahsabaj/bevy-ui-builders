//! Text input module - Gateway only

mod builder;
mod plugin;
mod systems;
mod types;
pub mod native_input;

// Public exports only
pub use builder::{TextInputBuilder, TextInputBuilderWithMarker, TextInputBuilderWithTwoMarkers, text_input};
pub use plugin::TextInputPlugin;
pub use systems::handle_clear_button_clicks;
pub use types::{
    TextInputFilter,
    InputFilter,
    InputTransform,
    ClearButtonTarget,
    TextInputFocus,
    FocusGroupId,
};
pub use native_input::{
    NativeTextInput,
    TextBuffer,
    SelectionState,
    TextInputVisual,
    CursorVisual,
    CursorStyle,
    TextInputSettings,
    TabBehavior,
    TextInputSubmitEvent,
    TextInputChangeEvent,
};
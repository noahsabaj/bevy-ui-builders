//! Text input interaction systems
//!
//! NOTE: These are legacy systems for bevy_simple_text_input compatibility.
//! The new native text input implementation in native_input.rs provides its own systems.

use bevy::prelude::*;
use super::types::*;

// Legacy systems preserved for potential migration support
// These are not registered in the plugin when using native text input

/// Handle clicking on text inputs to manage focus (LEGACY)
pub fn handle_text_input_focus(
    _commands: Commands,
    _interactions: Query<
        (Entity, &Interaction, &TextInputFocus),
        (Changed<Interaction>, /* With<TextInput> */)
    >,
    _all_inputs: Query<(Entity, &TextInputFocus), /* With<TextInput> */>,
) {
    // Legacy system - functionality moved to native_input.rs
}

/// Unfocus all text inputs when clicking outside any input (LEGACY)
pub fn handle_click_outside_unfocus(
    _commands: Commands,
    _mouse_button: Res<ButtonInput<MouseButton>>,
    _interactions: Query<&Interaction, /* With<TextInput> */>,
    _all_inputs: Query<Entity, /* With<TextInput> */>,
) {
    // Legacy system - functionality moved to native_input.rs handle_click_outside
}

/// Validate and filter text input changes based on TextInputFilter (LEGACY)
pub fn validate_text_input_changes(
    // Query parameters removed since we don't have bevy_simple_text_input components
) {
    // Legacy system - filtering is now handled in native_input.rs handle_keyboard_input
}

/// Handle clicks on clear buttons
pub fn handle_clear_button_clicks(
    button_query: Query<(&Interaction, &ClearButtonTarget), (Changed<Interaction>, With<Button>)>,
    mut text_inputs: Query<&mut super::native_input::TextBuffer, With<super::native_input::NativeTextInput>>,
) {
    for (interaction, target) in &button_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut buffer) = text_inputs.get_mut(target.0) {
                buffer.content.clear();
                buffer.cursor_pos = 0;
            }
        }
    }
}
//! Text input interaction systems

use bevy::prelude::*;
use super::types::ClearButtonTarget;

/// Handle clicks on clear buttons to clear their associated text input
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
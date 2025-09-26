//! Focus visual state management

use bevy::prelude::*;
use crate::styles::colors;
use super::super::components::*;

/// Maintain visual focus state (blue border when focused)
pub fn update_focus_visual(
    mut text_inputs: Query<
        (&TextBuffer, &mut BorderColor),
        (With<NativeTextInput>, Or<(Changed<TextBuffer>, Changed<BorderColor>)>)
    >,
) {
    for (buffer, mut border_color) in text_inputs.iter_mut() {
        // Apply focus border color when focused, normal border when not
        border_color.0 = if buffer.is_focused {
            colors::BORDER_FOCUS  // Blue border when focused
        } else {
            colors::BORDER_DEFAULT  // Default border when not focused
        };
    }
}
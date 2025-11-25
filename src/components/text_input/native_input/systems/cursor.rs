//! Cursor animation and visibility systems

use bevy::prelude::*;

use super::super::components::*;

/// Update cursor blinking
pub fn update_cursor_blink(
    mut text_inputs: Query<(&TextBuffer, &mut CursorVisual, Ref<TextBuffer>), With<NativeTextInput>>,
    time: Res<Time>,
) {
    for (buffer, mut cursor, buffer_ref) in text_inputs.iter_mut() {
        if !buffer.is_focused {
            cursor.visible = false;
            continue;
        }

        // If just gained focus, make cursor immediately visible
        if buffer_ref.is_changed() && buffer.is_focused {
            cursor.visible = true;
            cursor.blink_timer.reset();
        }

        cursor.blink_timer.tick(time.delta());

        if cursor.blink_timer.just_finished() {
            cursor.visible = !cursor.visible;
        }
    }
}
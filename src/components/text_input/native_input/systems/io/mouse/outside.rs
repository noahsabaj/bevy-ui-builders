//! Click outside handling to unfocus text inputs

use bevy::prelude::*;
use bevy::ecs::system::ParamSet;

use super::super::super::super::components::*;
use crate::components::text_input::types::TextInputFocus;

/// Handle click outside to unfocus text inputs
pub fn handle_click_outside(
    mouse: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<(&TextInputFocus, &Interaction), With<NativeTextInput>>,
        Query<(&mut TextBuffer, &mut SelectionState, &mut CursorVisual, &TextInputFocus), With<NativeTextInput>>,
    )>,
) {
    // Check if mouse was just clicked
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Check if any input is being clicked
    let clicked_input_exists: bool = {
        let p0 = param_set.p0();
        p0.iter().any(|(_, interaction)| *interaction == Interaction::Pressed)
    };

    // If clicking outside all inputs, unfocus all
    if !clicked_input_exists {
        let mut p1 = param_set.p1();
        for (mut buffer, mut selection, mut cursor_visual, _) in p1.iter_mut() {
            if buffer.is_focused {
                info!("Unfocusing input - clicked outside");
                buffer.is_focused = false;
                selection.clear();
                cursor_visual.visible = false;
            }
        }
    }
    // If clicking on an input, handle_mouse_input will handle focus management
}
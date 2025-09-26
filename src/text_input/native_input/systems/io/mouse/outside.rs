//! Click outside handling to unfocus text inputs

use bevy::prelude::*;
use bevy::ecs::system::ParamSet;

use super::super::super::super::components::*;
use crate::text_input::types::TextInputFocus;

/// Handle click outside to unfocus text inputs
pub fn handle_click_outside(
    mouse: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<(&TextInputFocus, &Interaction), With<NativeTextInput>>,
        Query<(&mut TextBuffer, &mut SelectionState, &TextInputFocus), With<NativeTextInput>>,
    )>,
) {
    // Check if mouse was just clicked
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Check if any input is being interacted with (by focus group)
    let mut interacted_groups = Vec::new();
    {
        let p0 = param_set.p0();
        for (focus, interaction) in p0.iter() {
            if *interaction == Interaction::Pressed {
                // Store the focus group that's being interacted with
                interacted_groups.push(focus.clone());
            }
        }
    }

    // If no input is being interacted with, unfocus all
    if interacted_groups.is_empty() {
        let mut p1 = param_set.p1();
        for (mut buffer, mut selection, _) in p1.iter_mut() {
            buffer.is_focused = false;
            selection.clear();
        }
    } else {
        // Unfocus inputs not in the interacted group (for exclusive groups)
        let mut p1 = param_set.p1();
        for (mut buffer, mut selection, focus) in p1.iter_mut() {
            // Check if this input's group is being interacted with
            let should_stay_focused = interacted_groups.iter().any(|interacted| match (focus, interacted) {
                (TextInputFocus::Independent, TextInputFocus::Independent) => false, // Independent inputs don't affect each other
                (TextInputFocus::ExclusiveGroup(g1), TextInputFocus::ExclusiveGroup(g2)) => g1 == g2,
                _ => false,
            });

            if !should_stay_focused && matches!(focus, TextInputFocus::ExclusiveGroup(_)) {
                buffer.is_focused = false;
                selection.clear();
            }
        }
    }
}
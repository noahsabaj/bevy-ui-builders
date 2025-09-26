//! Special key handling (Tab, Enter)

use bevy::prelude::*;
use super::super::super::super::components::{TextBuffer, SelectionState, TextInputSettings, NativeTextInput, CursorVisual};
use super::super::super::super::events::TextInputSubmitEvent;
use super::super::super::super::helpers::apply_edit;
use super::super::super::super::types::{TabBehavior, EditAction};
use crate::text_input::types::{TextInputFocus, FocusGroupId};

/// Handle Enter key
pub fn handle_enter(
    entity: Entity,
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    settings: &TextInputSettings,
    history: &mut super::super::super::super::components::UndoHistory,
    submit_events: &mut EventWriter<TextInputSubmitEvent>,
) {
    if !settings.multiline {
        // Submit
        submit_events.send(TextInputSubmitEvent {
            entity,
            text: buffer.content.clone(),
        });

        if !settings.retain_on_submit {
            buffer.content.clear();
            buffer.cursor_pos = 0;
            selection.clear();
        }
    } else {
        // Insert newline
        if let Some(op) = apply_edit(&EditAction::InsertChar('\n'), buffer, selection) {
            history.undo_stack.push_back(op);
            history.redo_stack.clear();
        }
    }
}

/// Handle Tab key navigation between text inputs
pub fn handle_tab_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut text_inputs: Query<(Entity, &mut TextBuffer, &mut SelectionState, &mut CursorVisual, &TextInputSettings, &TextInputFocus), With<NativeTextInput>>,
) {
    // Check if Tab was just pressed
    if !keyboard.just_pressed(KeyCode::Tab) {
        return;
    }

    let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    // Find currently focused input
    let mut current_entity = None;
    let mut all_inputs = Vec::new();

    for (entity, buffer, _, _, settings, focus) in text_inputs.iter() {
        if settings.tab_behavior != TabBehavior::NextField {
            continue;
        }

        all_inputs.push((entity, focus.clone()));
        if buffer.is_focused {
            current_entity = Some(entity);
        }
    }

    if let Some(current_entity) = current_entity {
        // Sort inputs by focus group and find next
        all_inputs.sort_by_key(|(_, focus)| match focus {
            TextInputFocus::Independent => 0,
            TextInputFocus::ExclusiveGroup(id) => match id {
                FocusGroupId::WorldConfig => 1,
                FocusGroupId::SaveDialog => 2,
                FocusGroupId::ModBrowser => 3,
                FocusGroupId::Custom(n) => 100 + n,
            },
        });

        let current_index = all_inputs.iter()
            .position(|(entity, _)| *entity == current_entity)
            .unwrap_or(0);

        let next_index = if shift_held {
            // Navigate backward
            if current_index == 0 {
                all_inputs.len() - 1
            } else {
                current_index - 1
            }
        } else {
            // Navigate forward
            (current_index + 1) % all_inputs.len()
        };

        let next_entity = all_inputs[next_index].0;

        // Unfocus current and focus next
        for (entity, mut buffer, mut selection, mut cursor_visual, _, _) in text_inputs.iter_mut() {
            if entity == current_entity {
                buffer.is_focused = false;
                selection.clear();
                cursor_visual.visible = false;
            } else if entity == next_entity {
                buffer.is_focused = true;
                cursor_visual.visible = true;
                cursor_visual.blink_timer.reset();
            }
        }
    }
}
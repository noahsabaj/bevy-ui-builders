//! Special key handling (Tab, Enter)

use bevy::prelude::*;
use super::super::super::super::components::{TextBuffer, SelectionState, TextInputSettings, NativeTextInput, CursorVisual};
use super::super::super::super::events::TextInputSubmitEvent;
use super::super::super::super::helpers::apply_edit;
use super::super::super::super::types::{TabBehavior, EditAction};
use crate::components::text_input::types::TextInputFocus;

/// Handle Enter key
pub fn handle_enter(
    entity: Entity,
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    settings: &TextInputSettings,
    history: &mut super::super::super::super::components::UndoHistory,
    submit_events: &mut MessageWriter<TextInputSubmitEvent>,
) {
    if !settings.multiline {
        // Submit
        submit_events.write(TextInputSubmitEvent {
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

    // Find currently focused input and its focus group
    let mut current_entity = None;
    let mut current_focus_group = None;
    let mut all_inputs = Vec::new();

    for (entity, buffer, _, _, settings, focus) in text_inputs.iter() {
        if settings.tab_behavior != TabBehavior::NextField {
            continue;
        }

        all_inputs.push((entity, focus.clone()));
        if buffer.is_focused {
            current_entity = Some(entity);
            current_focus_group = Some(focus.clone());
        }
    }

    if let Some(current_entity) = current_entity {
        // Filter inputs based on focus group
        let mut filtered_inputs: Vec<Entity> = match current_focus_group {
            Some(TextInputFocus::Independent) => {
                // Independent inputs can navigate to all other inputs
                all_inputs.iter().map(|(entity, _)| *entity).collect()
            }
            Some(TextInputFocus::ExclusiveGroup(ref group_id)) => {
                // Only navigate within the same focus group
                all_inputs.iter()
                    .filter_map(|(entity, focus)| {
                        match focus {
                            TextInputFocus::ExclusiveGroup(id) if id == group_id => Some(*entity),
                            _ => None,
                        }
                    })
                    .collect()
            }
            None => {
                // Fallback: navigate through all inputs
                all_inputs.iter().map(|(entity, _)| *entity).collect()
            }
        };

        // Sort filtered inputs by their entity ID for consistent ordering
        filtered_inputs.sort();

        // Find current position in filtered list
        let current_index = filtered_inputs.iter()
            .position(|&entity| entity == current_entity)
            .unwrap_or(0);

        // Calculate next index
        let next_index = if shift_held {
            // Navigate backward
            if current_index == 0 {
                filtered_inputs.len().saturating_sub(1)
            } else {
                current_index - 1
            }
        } else {
            // Navigate forward
            (current_index + 1) % filtered_inputs.len()
        };

        // Only navigate if there are other inputs to go to
        if !filtered_inputs.is_empty() && filtered_inputs.len() > 1 {
            let next_entity = filtered_inputs[next_index];

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
}
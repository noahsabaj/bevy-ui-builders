//! Mouse drag selection

use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use bevy::ui::RelativeCursorPosition;

use super::super::super::super::components::*;
use super::selection::calculate_char_index_from_position;

/// Handle mouse drag for text selection
pub fn handle_mouse_drag(
    mut text_inputs: Query<(
        &mut TextBuffer,
        &mut SelectionState,
        &RelativeCursorPosition,
        &Children,
    ), With<NativeTextInput>>,
    text_query: Query<&TextLayoutInfo, With<TextInputInner>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    // Only process drag if mouse is held down
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    for (mut buffer, mut selection, cursor_pos, children) in text_inputs.iter_mut() {
        if !buffer.is_focused {
            continue;
        }

        // Find the text inner entity
        let mut text_entity = None;
        for child in children.iter() {
            if text_query.get(child).is_ok() {
                text_entity = Some(child);
                break;
            }
        }

        if let Some(text_entity) = text_entity {
            if let Ok(text_layout) = text_query.get(text_entity) {
                if let Some(normalized_pos) = cursor_pos.normalized {
                    // Calculate current mouse position in text
                    let char_index = calculate_char_index_from_position(
                        normalized_pos,
                        text_layout,
                        &buffer.content,
                    );

                    // If we have an anchor, update the selection
                    if let Some(anchor) = selection.anchor {
                        if char_index != anchor {
                            selection.anchor = Some(anchor);
                            selection.cursor = char_index;
                        } else {
                            // No selection if cursor is at anchor
                            selection.clear();
                        }
                    }
                }
            }
        }
    }
}
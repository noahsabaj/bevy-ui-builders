//! Text editing operations (insert, delete)

use bevy::prelude::*;
use bevy::input::keyboard::Key;
use super::super::super::super::components::{TextBuffer, SelectionState, UndoHistory, TextInputSettings};
use super::super::super::super::helpers::apply_edit;
use super::super::super::super::types::EditAction;

/// Handle character input
pub fn handle_character_input(
    key: &Key,
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
    settings: &TextInputSettings,
) {
    match key {
        Key::Character(text) => {
            info!("Received character input: '{}'", text);

            for ch in text.chars() {
                // Check max length before each character insertion
                if let Some(max) = settings.max_length {
                    let current_len = buffer.content.chars().count();
                    let selection_len = if selection.has_selection() {
                        if let Some((start, end)) = selection.range() {
                            end - start
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    // Calculate what the new length would be after insertion
                    let new_len = current_len - selection_len + 1;
                    if new_len > max {
                        info!("Max length {} reached, ignoring character", max);
                        return;  // Don't insert if it would exceed max length
                    }
                }

                info!("Processing character: '{}', buffer before: '{}', cursor: {}", ch, buffer.content, buffer.cursor_pos);
                if let Some(op) = apply_edit(&EditAction::InsertChar(ch), buffer, selection) {
                    history.undo_stack.push_back(op);
                    history.redo_stack.clear();
                    info!("After edit - buffer: '{}', cursor: {}", buffer.content, buffer.cursor_pos);
                }
            }
        }
        Key::Space => {
            // Check max length for space as well
            if let Some(max) = settings.max_length {
                let current_len = buffer.content.chars().count();
                let selection_len = if selection.has_selection() {
                    if let Some((start, end)) = selection.range() {
                        end - start
                    } else {
                        0
                    }
                } else {
                    0
                };

                let new_len = current_len - selection_len + 1;
                if new_len > max {
                    info!("Max length {} reached, ignoring space", max);
                    return;
                }
            }

            // Handle space separately since it's not a Character variant
            if let Some(op) = apply_edit(&EditAction::InsertChar(' '), buffer, selection) {
                history.undo_stack.push_back(op);
                history.redo_stack.clear();
            }
        }
        _ => {}
    }
}

/// Handle backspace deletion
pub fn handle_backspace(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = apply_edit(&EditAction::DeleteBackward, buffer, selection) {
        history.undo_stack.push_back(op);
        history.redo_stack.clear();
    }
}

/// Handle delete key
pub fn handle_delete(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = apply_edit(&EditAction::DeleteForward, buffer, selection) {
        history.undo_stack.push_back(op);
        history.redo_stack.clear();
    }
}

/// Handle word deletion (Ctrl+Backspace/Delete)
pub fn handle_delete_word_backward(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = apply_edit(&EditAction::DeleteWordBackward, buffer, selection) {
        history.undo_stack.push_back(op);
        history.redo_stack.clear();
    }
}

pub fn handle_delete_word_forward(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = apply_edit(&EditAction::DeleteWordForward, buffer, selection) {
        history.undo_stack.push_back(op);
        history.redo_stack.clear();
    }
}

/// Handle delete to line start (Ctrl+Shift+Backspace)
pub fn handle_delete_to_line_start(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    use super::super::super::super::types::OperationType;
    use super::super::super::super::components::EditOperation;

    if buffer.cursor_pos > 0 {
        let deleted = buffer.content.chars().take(buffer.cursor_pos).collect::<String>();
        buffer.content = buffer.content.chars().skip(buffer.cursor_pos).collect();
        let op = EditOperation {
            op_type: OperationType::Delete {
                pos: 0,
                text: deleted,
            },
            cursor_before: buffer.cursor_pos,
            cursor_after: 0,
        };
        buffer.cursor_pos = 0;
        selection.clear();
        history.undo_stack.push_back(op);
    }
}

/// Handle delete to line end (Ctrl+Shift+Delete)
pub fn handle_delete_to_line_end(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    use super::super::super::super::types::OperationType;
    use super::super::super::super::components::EditOperation;

    if buffer.cursor_pos < buffer.content.chars().count() {
        let deleted = buffer.content.chars().skip(buffer.cursor_pos).collect::<String>();
        buffer.content = buffer.content.chars().take(buffer.cursor_pos).collect();
        let op = EditOperation {
            op_type: OperationType::Delete {
                pos: buffer.cursor_pos,
                text: deleted,
            },
            cursor_before: buffer.cursor_pos,
            cursor_after: buffer.cursor_pos,
        };
        selection.clear();
        history.undo_stack.push_back(op);
    }
}
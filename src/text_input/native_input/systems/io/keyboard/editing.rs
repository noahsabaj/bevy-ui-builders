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
            // Check max length
            if let Some(max) = settings.max_length {
                if buffer.content.chars().count() >= max && !selection.has_selection() {
                    return;
                }
            }

            for ch in text.chars() {
                info!("Processing character: '{}', buffer before: '{}', cursor: {}", ch, buffer.content, buffer.cursor_pos);
                if let Some(op) = apply_edit(&EditAction::InsertChar(ch), buffer, selection) {
                    history.undo_stack.push_back(op);
                    history.redo_stack.clear();
                    info!("After edit - buffer: '{}', cursor: {}", buffer.content, buffer.cursor_pos);
                }
            }
        }
        Key::Space => {
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
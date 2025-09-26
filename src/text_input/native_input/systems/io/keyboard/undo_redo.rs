//! Undo/Redo operations

use super::super::super::super::components::{TextBuffer, SelectionState, UndoHistory};
use super::super::super::super::types::OperationType;

/// Handle undo operation (Ctrl+Z)
pub fn handle_undo(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = history.undo_stack.pop_back() {
        // Apply the inverse operation
        match &op.op_type {
            OperationType::Insert { pos: position, text } => {
                // Remove the inserted text
                let end_pos = *position + text.chars().count();
                buffer.content = format!(
                    "{}{}",
                    buffer.content.chars().take(*position).collect::<String>(),
                    buffer.content.chars().skip(end_pos).collect::<String>()
                );
                buffer.cursor_pos = *position;
            }
            OperationType::Delete { pos: position, text } => {
                // Re-insert the deleted text
                let before = buffer.content.chars().take(*position).collect::<String>();
                let after = buffer.content.chars().skip(*position).collect::<String>();
                buffer.content = format!("{}{}{}", before, text, after);
                buffer.cursor_pos = *position + text.chars().count();
            }
            OperationType::Replace { .. } => {
                // Handle replace if needed
            }
        }
        history.redo_stack.push_back(op);
        selection.clear();
    }
}

/// Handle redo operation (Ctrl+Y or Ctrl+Shift+Z)
pub fn handle_redo(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(op) = history.redo_stack.pop_back() {
        // Re-apply the operation
        match op.op_type.clone() {
            OperationType::Insert { pos: position, ref text } => {
                let before = buffer.content.chars().take(position).collect::<String>();
                let after = buffer.content.chars().skip(position).collect::<String>();
                buffer.content = format!("{}{}{}", before, text, after);
                buffer.cursor_pos = position + text.chars().count();
            }
            OperationType::Delete { pos: position, ref text } => {
                let end_pos = position + text.chars().count();
                buffer.content = format!(
                    "{}{}",
                    buffer.content.chars().take(position).collect::<String>(),
                    buffer.content.chars().skip(end_pos).collect::<String>()
                );
                buffer.cursor_pos = position;
            }
            OperationType::Replace { .. } => {
                // Handle replace if needed
            }
        }
        history.undo_stack.push_back(op);
        selection.clear();
    }
}
//! Clipboard operations (cut, copy, paste)

use super::super::super::super::components::{TextBuffer, SelectionState, UndoHistory};
use super::super::super::super::helpers::{apply_edit, get_selected_text};
use super::super::super::super::types::EditAction;

/// Handle cut operation (Ctrl+X)
pub fn handle_cut(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if selection.has_selection() {
        // Get selected text for clipboard
        if let Some(selected) = get_selected_text(buffer, selection) {
            // Copy to clipboard
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text(&selected);
            }
        }

        // Delete the selected text
        if let Some(op) = apply_edit(&EditAction::DeleteSelection, buffer, selection) {
            history.undo_stack.push_back(op);
            history.redo_stack.clear();
        }
    }
}

/// Handle copy operation (Ctrl+C)
pub fn handle_copy(
    buffer: &TextBuffer,
    selection: &SelectionState,
) {
    if selection.has_selection() {
        if let Some(selected) = get_selected_text(buffer, selection) {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text(&selected);
            }
        }
    }
}

/// Handle paste operation (Ctrl+V)
pub fn handle_paste(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(text) = clipboard.get_text() {
            // Apply paste operation
            if let Some(op) = apply_edit(&EditAction::PasteFromClipboard(text), buffer, selection) {
                history.undo_stack.push_back(op);
                history.redo_stack.clear();
            }
        }
    }
}
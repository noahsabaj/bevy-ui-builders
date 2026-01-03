//! Clipboard operations (cut, copy, paste)
//!
//! Clipboard support requires the `clipboard` feature and is not available on WASM targets.
//! When clipboard is unavailable, these operations are no-ops.

use super::super::super::super::components::{TextBuffer, SelectionState, UndoHistory};
use super::super::super::super::helpers::{apply_edit, get_selected_text};
use super::super::super::super::types::EditAction;

/// Copy text to system clipboard (if available)
#[cfg(feature = "clipboard")]
fn copy_to_clipboard(text: &str) {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text);
    }
}

/// Copy text to system clipboard (no-op when clipboard unavailable)
#[cfg(not(feature = "clipboard"))]
fn copy_to_clipboard(_text: &str) {
    // Clipboard not available (WASM or feature disabled)
}

/// Get text from system clipboard (if available)
#[cfg(feature = "clipboard")]
fn get_from_clipboard() -> Option<String> {
    arboard::Clipboard::new()
        .ok()
        .and_then(|mut cb| cb.get_text().ok())
}

/// Get text from system clipboard (returns None when clipboard unavailable)
#[cfg(not(feature = "clipboard"))]
fn get_from_clipboard() -> Option<String> {
    // Clipboard not available (WASM or feature disabled)
    None
}

/// Handle cut operation (Ctrl+X)
pub fn handle_cut(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if selection.has_selection() {
        // Get selected text for clipboard
        if let Some(selected) = get_selected_text(buffer, selection) {
            copy_to_clipboard(&selected);
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
            copy_to_clipboard(&selected);
        }
    }
}

/// Handle paste operation (Ctrl+V)
pub fn handle_paste(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    history: &mut UndoHistory,
) {
    if let Some(text) = get_from_clipboard() {
        // Apply paste operation
        if let Some(op) = apply_edit(&EditAction::PasteFromClipboard(text), buffer, selection) {
            history.undo_stack.push_back(op);
            history.redo_stack.clear();
        }
    }
}

//! Selection handling operations

use super::super::super::super::components::{TextBuffer, SelectionState};

/// Handle select all (Ctrl+A)
pub fn handle_select_all(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
) {
    selection.anchor = Some(0);
    selection.cursor = buffer.content.chars().count();
    buffer.cursor_pos = buffer.content.chars().count();
}
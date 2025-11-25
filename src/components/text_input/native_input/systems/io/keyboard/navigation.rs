//! Navigation key handling (arrows, home, end)

use super::super::super::super::components::{TextBuffer, SelectionState};

/// Handle arrow key navigation
pub fn handle_arrow_left(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    shift: bool,
) {
    if shift {
        // Extend selection
        if selection.anchor.is_none() {
            selection.start_selection(buffer.cursor_pos);
        }
        buffer.cursor_pos = buffer.cursor_pos.saturating_sub(1);
        selection.update_selection(buffer.cursor_pos);
    } else {
        // Just move cursor
        if selection.has_selection() {
            // Move to start of selection
            if let Some((start, _)) = selection.range() {
                buffer.cursor_pos = start;
            }
            selection.clear();
        } else {
            buffer.cursor_pos = buffer.cursor_pos.saturating_sub(1);
        }
    }
}

/// Handle right arrow navigation
pub fn handle_arrow_right(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    shift: bool,
) {
    if shift {
        // Extend selection
        if selection.anchor.is_none() {
            selection.start_selection(buffer.cursor_pos);
        }
        buffer.cursor_pos = (buffer.cursor_pos + 1).min(buffer.content.chars().count());
        selection.update_selection(buffer.cursor_pos);
    } else {
        // Just move cursor
        if selection.has_selection() {
            // Move to end of selection
            if let Some((_, end)) = selection.range() {
                buffer.cursor_pos = end;
            }
            selection.clear();
        } else {
            buffer.cursor_pos = (buffer.cursor_pos + 1).min(buffer.content.chars().count());
        }
    }
}

/// Handle Home key navigation
pub fn handle_home(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    shift: bool,
) {
    if shift {
        // Extend selection to start
        if selection.anchor.is_none() {
            selection.start_selection(buffer.cursor_pos);
        }
        buffer.cursor_pos = 0;
        selection.update_selection(0);
    } else {
        buffer.cursor_pos = 0;
        selection.clear();
    }
}

/// Handle End key navigation
pub fn handle_end(
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
    shift: bool,
) {
    if shift {
        // Extend selection to end
        if selection.anchor.is_none() {
            selection.start_selection(buffer.cursor_pos);
        }
        buffer.cursor_pos = buffer.content.chars().count();
        selection.update_selection(buffer.cursor_pos);
    } else {
        buffer.cursor_pos = buffer.content.chars().count();
        selection.clear();
    }
}
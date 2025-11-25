//! Helper functions for text manipulation and navigation

use bevy::prelude::*;
use super::components::{TextBuffer, SelectionState, EditOperation};
use super::types::{NavigationAction, EditAction, OperationType};

/// Convert character index to byte index in a UTF-8 string
pub fn char_to_byte_index(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(i, _)| i)
        .unwrap_or_else(|| text.len())
}

/// Convert byte index to character index in a UTF-8 string
pub fn byte_to_char_index(text: &str, byte_index: usize) -> usize {
    text.char_indices()
        .position(|(i, _)| i >= byte_index)
        .unwrap_or_else(|| text.chars().count())
}

/// Find the start of the previous word from the given position
pub fn find_word_boundary_backward(text: &str, pos: usize) -> usize {
    if pos == 0 {
        return 0;
    }

    let chars: Vec<char> = text.chars().collect();
    let mut idx = pos.min(chars.len());

    // Skip any whitespace we're currently in
    while idx > 0 && chars[idx - 1].is_whitespace() {
        idx -= 1;
    }

    // Skip word characters until we hit whitespace or punctuation
    while idx > 0 && !chars[idx - 1].is_whitespace() && chars[idx - 1].is_alphanumeric() {
        idx -= 1;
    }

    // If we're at punctuation, skip it as a separate word
    if idx > 0 && !chars[idx - 1].is_whitespace() && !chars[idx - 1].is_alphanumeric() {
        while idx > 0 && !chars[idx - 1].is_whitespace() && !chars[idx - 1].is_alphanumeric() {
            idx -= 1;
        }
    }

    idx
}

/// Find the end of the next word from the given position
pub fn find_word_boundary_forward(text: &str, pos: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    if pos >= len {
        return len;
    }

    let mut idx = pos;

    // Skip current word characters
    if idx < len && chars[idx].is_alphanumeric() {
        while idx < len && chars[idx].is_alphanumeric() {
            idx += 1;
        }
    }
    // Skip punctuation as separate word
    else if idx < len && !chars[idx].is_whitespace() {
        while idx < len && !chars[idx].is_whitespace() && !chars[idx].is_alphanumeric() {
            idx += 1;
        }
    }

    // Skip whitespace
    while idx < len && chars[idx].is_whitespace() {
        idx += 1;
    }

    idx
}

/// Find the start of the current line
pub fn find_line_start(text: &str, pos: usize) -> usize {
    let byte_pos = char_to_byte_index(text, pos);

    // Find the last newline before our position
    if let Some(newline_pos) = text[..byte_pos].rfind('\n') {
        byte_to_char_index(text, newline_pos + 1)
    } else {
        0
    }
}

/// Find the end of the current line
pub fn find_line_end(text: &str, pos: usize) -> usize {
    let byte_pos = char_to_byte_index(text, pos);

    // Find the next newline after our position
    if let Some(newline_pos) = text[byte_pos..].find('\n') {
        byte_to_char_index(text, byte_pos + newline_pos)
    } else {
        text.chars().count()
    }
}

/// Apply a navigation action to cursor position
pub fn apply_navigation(
    action: NavigationAction,
    cursor_pos: usize,
    content: &str,
) -> usize {
    let char_count = content.chars().count();

    match action {
        NavigationAction::CharLeft => cursor_pos.saturating_sub(1),
        NavigationAction::CharRight => (cursor_pos + 1).min(char_count),
        NavigationAction::LineStart => {
            // Find start of current line
            let byte_pos = char_to_byte_index(content, cursor_pos);
            let line_start = content[..byte_pos]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            byte_to_char_index(content, line_start)
        }
        NavigationAction::LineEnd => {
            // Find end of current line
            let byte_pos = char_to_byte_index(content, cursor_pos);
            let line_end = content[byte_pos..]
                .find('\n')
                .map(|i| byte_pos + i)
                .unwrap_or_else(|| content.len());
            byte_to_char_index(content, line_end)
        }
        NavigationAction::WordLeft => {
            // Move to start of previous word
            if cursor_pos == 0 {
                return 0;
            }

            let mut pos = cursor_pos - 1;
            let chars: Vec<char> = content.chars().collect();

            // Skip whitespace
            while pos > 0 && chars[pos].is_whitespace() {
                pos -= 1;
            }

            // Skip word characters
            while pos > 0 && !chars[pos - 1].is_whitespace() {
                pos -= 1;
            }

            pos
        }
        NavigationAction::WordRight => {
            // Move to end of next word
            if cursor_pos >= char_count {
                return char_count;
            }

            let mut pos = cursor_pos;
            let chars: Vec<char> = content.chars().collect();

            // Skip word characters
            while pos < char_count && !chars[pos].is_whitespace() {
                pos += 1;
            }

            // Skip whitespace
            while pos < char_count && chars[pos].is_whitespace() {
                pos += 1;
            }

            pos
        }
        NavigationAction::DocumentStart => 0,
        NavigationAction::DocumentEnd => char_count,
        NavigationAction::LineUp | NavigationAction::LineDown => {
            // For multiline navigation (simplified for now)
            cursor_pos
        }
    }
}

/// Get the selected text from the buffer
pub fn get_selected_text(buffer: &TextBuffer, selection: &SelectionState) -> Option<String> {
    if let Some((start, end)) = selection.range() {
        let start_byte = char_to_byte_index(&buffer.content, start);
        let end_byte = char_to_byte_index(&buffer.content, end);
        Some(buffer.content[start_byte..end_byte].to_string())
    } else {
        None
    }
}

/// Apply an edit action to the text buffer
pub fn apply_edit(
    action: &EditAction,
    buffer: &mut TextBuffer,
    selection: &mut SelectionState,
) -> Option<EditOperation> {
    match action {
        EditAction::InsertChar(ch) => {
            info!("apply_edit InsertChar: '{}' at pos {}", ch, buffer.cursor_pos);
            // Delete selection first if exists
            if selection.has_selection() {
                if let Some(_op) = apply_edit(&EditAction::DeleteSelection, buffer, selection) {
                    // Store the delete operation separately if needed
                }
            }

            let byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos);
            info!("Inserting '{}' at byte pos {}", ch, byte_pos);
            buffer.content.insert(byte_pos, *ch);
            info!("Buffer after insert: '{}'", buffer.content);

            let op = EditOperation {
                op_type: OperationType::Insert {
                    pos: buffer.cursor_pos,
                    text: ch.to_string(),
                },
                cursor_before: buffer.cursor_pos,
                cursor_after: buffer.cursor_pos + 1,
            };

            buffer.cursor_pos += 1;
            selection.clear();
            info!("Returning Some(op) from apply_edit");

            Some(op)
        }
        EditAction::InsertString(text) => {
            // Delete selection first if exists
            if selection.has_selection() {
                if let Some(_op) = apply_edit(&EditAction::DeleteSelection, buffer, selection) {
                    // Store the delete operation separately if needed
                }
            }

            let byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos);
            let char_count = text.chars().count();
            buffer.content.insert_str(byte_pos, text);

            let op = EditOperation {
                op_type: OperationType::Insert {
                    pos: buffer.cursor_pos,
                    text: text.clone(),
                },
                cursor_before: buffer.cursor_pos,
                cursor_after: buffer.cursor_pos + char_count,
            };

            buffer.cursor_pos += char_count;
            selection.clear();

            Some(op)
        }
        EditAction::DeleteBackward => {
            if buffer.cursor_pos > 0 {
                let byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos - 1);
                let next_byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                let deleted = buffer.content[byte_pos..next_byte_pos].to_string();

                buffer.content.drain(byte_pos..next_byte_pos);

                let op = EditOperation {
                    op_type: OperationType::Delete {
                        pos: buffer.cursor_pos - 1,
                        text: deleted,
                    },
                    cursor_before: buffer.cursor_pos,
                    cursor_after: buffer.cursor_pos - 1,
                };

                buffer.cursor_pos -= 1;
                selection.clear();

                Some(op)
            } else {
                None
            }
        }
        EditAction::DeleteSelection => {
            if let Some((start, end)) = selection.range() {
                let start_byte = char_to_byte_index(&buffer.content, start);
                let end_byte = char_to_byte_index(&buffer.content, end);
                let deleted = buffer.content[start_byte..end_byte].to_string();

                buffer.content.drain(start_byte..end_byte);

                let op = EditOperation {
                    op_type: OperationType::Delete {
                        pos: start,
                        text: deleted,
                    },
                    cursor_before: buffer.cursor_pos,
                    cursor_after: start,
                };

                buffer.cursor_pos = start;
                selection.clear();

                Some(op)
            } else {
                None
            }
        }
        EditAction::DeleteForward => {
            let char_count = buffer.content.chars().count();
            if buffer.cursor_pos < char_count {
                let byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                let next_byte_pos = char_to_byte_index(&buffer.content, buffer.cursor_pos + 1);
                let deleted = buffer.content[byte_pos..next_byte_pos].to_string();

                buffer.content.drain(byte_pos..next_byte_pos);

                let op = EditOperation {
                    op_type: OperationType::Delete {
                        pos: buffer.cursor_pos,
                        text: deleted,
                    },
                    cursor_before: buffer.cursor_pos,
                    cursor_after: buffer.cursor_pos,
                };

                selection.clear();

                Some(op)
            } else {
                None
            }
        }
        EditAction::DeleteWordBackward => {
            if buffer.cursor_pos > 0 {
                let word_start = find_word_boundary_backward(&buffer.content, buffer.cursor_pos);
                if word_start < buffer.cursor_pos {
                    let start_byte = char_to_byte_index(&buffer.content, word_start);
                    let end_byte = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                    let deleted = buffer.content[start_byte..end_byte].to_string();

                    buffer.content.drain(start_byte..end_byte);

                    let op = EditOperation {
                        op_type: OperationType::Delete {
                            pos: word_start,
                            text: deleted,
                        },
                        cursor_before: buffer.cursor_pos,
                        cursor_after: word_start,
                    };

                    buffer.cursor_pos = word_start;
                    selection.clear();

                    Some(op)
                } else {
                    None
                }
            } else {
                None
            }
        }
        EditAction::DeleteWordForward => {
            let char_count = buffer.content.chars().count();
            if buffer.cursor_pos < char_count {
                let word_end = find_word_boundary_forward(&buffer.content, buffer.cursor_pos);
                if word_end > buffer.cursor_pos {
                    let start_byte = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                    let end_byte = char_to_byte_index(&buffer.content, word_end);
                    let deleted = buffer.content[start_byte..end_byte].to_string();

                    buffer.content.drain(start_byte..end_byte);

                    let op = EditOperation {
                        op_type: OperationType::Delete {
                            pos: buffer.cursor_pos,
                            text: deleted,
                        },
                        cursor_before: buffer.cursor_pos,
                        cursor_after: buffer.cursor_pos,
                    };

                    selection.clear();

                    Some(op)
                } else {
                    None
                }
            } else {
                None
            }
        }
        EditAction::DeleteToLineStart => {
            if buffer.cursor_pos > 0 {
                let line_start = find_line_start(&buffer.content, buffer.cursor_pos);
                if line_start < buffer.cursor_pos {
                    let start_byte = char_to_byte_index(&buffer.content, line_start);
                    let end_byte = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                    let deleted = buffer.content[start_byte..end_byte].to_string();

                    buffer.content.drain(start_byte..end_byte);

                    let op = EditOperation {
                        op_type: OperationType::Delete {
                            pos: line_start,
                            text: deleted,
                        },
                        cursor_before: buffer.cursor_pos,
                        cursor_after: line_start,
                    };

                    buffer.cursor_pos = line_start;
                    selection.clear();

                    Some(op)
                } else {
                    None
                }
            } else {
                None
            }
        }
        EditAction::DeleteToLineEnd => {
            let char_count = buffer.content.chars().count();
            if buffer.cursor_pos < char_count {
                let line_end = find_line_end(&buffer.content, buffer.cursor_pos);
                if line_end > buffer.cursor_pos {
                    let start_byte = char_to_byte_index(&buffer.content, buffer.cursor_pos);
                    let end_byte = char_to_byte_index(&buffer.content, line_end);
                    let deleted = buffer.content[start_byte..end_byte].to_string();

                    buffer.content.drain(start_byte..end_byte);

                    let op = EditOperation {
                        op_type: OperationType::Delete {
                            pos: buffer.cursor_pos,
                            text: deleted,
                        },
                        cursor_before: buffer.cursor_pos,
                        cursor_after: buffer.cursor_pos,
                    };

                    selection.clear();

                    Some(op)
                } else {
                    None
                }
            } else {
                None
            }
        }
        EditAction::CutSelection => {
            // Cut is delete + copy (copy is handled in the system)
            apply_edit(&EditAction::DeleteSelection, buffer, selection)
        }
        EditAction::CopySelection => {
            // Copy doesn't modify the buffer, just returns None
            // The actual copying is handled in the system with clipboard access
            None
        }
        EditAction::PasteFromClipboard(text) => {
            // Paste is just an insert string operation
            apply_edit(&EditAction::InsertString(text.clone()), buffer, selection)
        }
    }
}
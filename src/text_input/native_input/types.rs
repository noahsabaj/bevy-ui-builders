//! Type definitions for native text input

/// Cursor rendering style
#[derive(Clone, Copy, Debug)]
pub enum CursorStyle {
    /// Vertical line cursor (default)
    Line,
    /// Block cursor (covers character)
    Block,
    /// Underline cursor
    Underline,
}

/// Type of edit operation for undo/redo
#[derive(Clone, Debug)]
pub enum OperationType {
    /// Insert text at position
    Insert { pos: usize, text: String },
    /// Delete text range
    Delete { pos: usize, text: String },
    /// Replace text range
    Replace { pos: usize, old: String, new: String },
}

/// Tab key behavior configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabBehavior {
    /// Tab moves to next field
    NextField,
    /// Tab inserts a tab character
    InsertTab,
    /// Tab inserts spaces
    InsertSpaces(usize),
}

/// Navigation actions for cursor movement
#[derive(Debug, Clone, Copy)]
pub enum NavigationAction {
    /// Move cursor left
    CharLeft,
    /// Move cursor right
    CharRight,
    /// Move cursor up (multiline)
    LineUp,
    /// Move cursor down (multiline)
    LineDown,
    /// Move to start of line
    LineStart,
    /// Move to end of line
    LineEnd,
    /// Move to start of word
    WordLeft,
    /// Move to end of word
    WordRight,
    /// Move to start of document
    DocumentStart,
    /// Move to end of document
    DocumentEnd,
}

/// Edit actions for text manipulation
#[derive(Debug, Clone)]
pub enum EditAction {
    /// Insert character at cursor
    InsertChar(char),
    /// Insert string at cursor
    InsertString(String),
    /// Delete character before cursor
    DeleteBackward,
    /// Delete character after cursor
    DeleteForward,
    /// Delete word before cursor
    DeleteWordBackward,
    /// Delete word after cursor
    DeleteWordForward,
    /// Delete to start of line
    DeleteToLineStart,
    /// Delete to end of line
    DeleteToLineEnd,
    /// Delete selected text
    DeleteSelection,
    /// Cut selected text to clipboard
    CutSelection,
    /// Copy selected text to clipboard
    CopySelection,
    /// Paste text from clipboard
    PasteFromClipboard(String),
}
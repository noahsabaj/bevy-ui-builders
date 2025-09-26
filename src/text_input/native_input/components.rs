//! Component definitions for native text input

use bevy::prelude::*;
use std::collections::VecDeque;
use super::helpers::{char_to_byte_index};
use super::types::{CursorStyle, OperationType, TabBehavior};

/// The main text input component - this is all you need to spawn
#[derive(Component, Default)]
pub struct NativeTextInput;

/// The internal text buffer and cursor management
#[derive(Component, Default)]
pub struct TextBuffer {
    /// The actual text content
    pub content: String,
    /// Current cursor position (in chars, not bytes)
    pub cursor_pos: usize,
    /// Whether the input is currently focused
    pub is_focused: bool,
}

/// Text selection state
#[derive(Component, Default)]
pub struct SelectionState {
    /// Selection anchor (where selection started)
    pub anchor: Option<usize>,
    /// Current selection end (moves with cursor)
    pub cursor: usize,
}

impl SelectionState {
    /// Get the normalized selection range (start, end)
    pub fn range(&self) -> Option<(usize, usize)> {
        self.anchor.map(|anchor| {
            if anchor < self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some() && self.anchor != Some(self.cursor)
    }

    /// Clear the selection
    pub fn clear(&mut self) {
        self.anchor = None;
    }

    /// Start a new selection
    pub fn start_selection(&mut self, pos: usize) {
        self.anchor = Some(pos);
        self.cursor = pos;
    }

    /// Update selection end
    pub fn update_selection(&mut self, pos: usize) {
        if self.anchor.is_none() {
            self.anchor = Some(pos);
        }
        self.cursor = pos;
    }

    /// Get selected text from buffer
    pub fn get_selected_text<'a>(&self, content: &'a str) -> Option<&'a str> {
        self.range().map(|(start, end)| {
            let start_byte = char_to_byte_index(content, start);
            let end_byte = char_to_byte_index(content, end);
            &content[start_byte..end_byte]
        })
    }
}

/// Visual settings for the input
#[derive(Component)]
pub struct TextInputVisual {
    /// Font settings
    pub font: TextFont,
    /// Text color
    pub text_color: Color,
    /// Selection color
    pub selection_color: Color,
    /// Cursor color
    pub cursor_color: Color,
    /// Placeholder text
    pub placeholder: String,
    /// Placeholder color
    pub placeholder_color: Color,
    /// Whether to mask input (for passwords)
    pub mask_char: Option<char>,
}

impl Default for TextInputVisual {
    fn default() -> Self {
        Self {
            font: TextFont::default(),
            text_color: Color::WHITE,
            selection_color: Color::srgba(0.3, 0.5, 0.8, 0.3),
            cursor_color: Color::WHITE,
            placeholder: String::new(),
            placeholder_color: Color::srgba(0.5, 0.5, 0.5, 0.5),
            mask_char: None,
        }
    }
}

/// Cursor visual state
#[derive(Component)]
pub struct CursorVisual {
    /// Timer for blinking animation
    pub blink_timer: Timer,
    /// Whether cursor is currently visible
    pub visible: bool,
    /// Cursor style
    pub style: CursorStyle,
    /// Entity of the visual cursor (if spawned)
    pub cursor_entity: Option<Entity>,
    /// Entities of selection overlays (for rendering selection highlights)
    pub selection_entities: Vec<Entity>,
}

impl Default for CursorVisual {
    fn default() -> Self {
        Self {
            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            visible: true,
            style: CursorStyle::Line,
            cursor_entity: None,
            selection_entities: Vec::new(),
        }
    }
}

/// Scroll state for overflow handling
#[derive(Component, Default)]
pub struct ScrollViewport {
    /// Horizontal scroll offset
    pub offset_x: f32,
    /// Vertical scroll offset (for multiline)
    pub offset_y: f32,
}

/// Undo/redo history
#[derive(Component)]
pub struct UndoHistory {
    /// Stack of undo operations
    pub undo_stack: VecDeque<EditOperation>,
    /// Stack of redo operations
    pub redo_stack: VecDeque<EditOperation>,
    /// Maximum history size
    pub max_size: usize,
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_size: 100,
        }
    }
}

/// Represents a single edit operation
#[derive(Clone, Debug)]
pub struct EditOperation {
    /// The operation type
    pub op_type: OperationType,
    /// Cursor position before the operation
    pub cursor_before: usize,
    /// Cursor position after the operation
    pub cursor_after: usize,
}

/// Inner text node marker
#[derive(Component)]
pub struct TextInputInner;

/// Marker component for the selection overlay
#[derive(Component)]
pub struct TextInputSelection {
    /// The input entity this selection belongs to
    pub input_entity: Entity,
}

/// Text input settings/configuration
#[derive(Component)]
pub struct TextInputSettings {
    /// Whether the input is multiline
    pub multiline: bool,
    /// Maximum length in characters
    pub max_length: Option<usize>,
    /// Whether to retain text on submit
    pub retain_on_submit: bool,
    /// Whether the input is read-only
    pub read_only: bool,
    /// Tab behavior
    pub tab_behavior: TabBehavior,
}

impl Default for TextInputSettings {
    fn default() -> Self {
        Self {
            multiline: false,
            max_length: None,
            retain_on_submit: false,
            read_only: false,
            tab_behavior: TabBehavior::NextField,
        }
    }
}
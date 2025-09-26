//! Native text input module - Gateway only
//!
//! This module provides a complete, feature-rich text input system with:
//! - Full keyboard navigation
//! - Text selection
//! - Clipboard support
//! - Undo/redo
//! - Multi-line support

// Private submodules - NEVER use pub mod
mod components;
mod types;
mod events;
mod helpers;
mod systems;

// Public exports - controlled API surface
pub use components::{
    NativeTextInput,
    TextBuffer,
    SelectionState,
    TextInputVisual,
    CursorVisual,
    TextInputSettings,
    TextInputInner,
    ScrollViewport,
    UndoHistory,
    EditOperation,
    TextInputCursor,
    TextInputSelection,
};

pub use types::{
    CursorStyle,
    TabBehavior,
    NavigationAction,
    EditAction,
    OperationType,
};

pub use events::{
    TextInputSubmitEvent,
    TextInputChangeEvent,
};

// System exports for plugin registration
pub use systems::{
    init_text_input,
    handle_keyboard_input,
    handle_mouse_input,
    handle_mouse_drag,
    handle_click_outside,
    handle_tab_navigation,
    update_cursor_blink,
    render_text,
    update_cursor_visual,
    render_selection,
};

// Helper function exports (if needed externally)
pub use helpers::{
    char_to_byte_index,
    apply_navigation,
    apply_edit,
    get_selected_text,
};
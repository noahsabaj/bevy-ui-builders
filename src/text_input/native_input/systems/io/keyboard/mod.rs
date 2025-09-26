//! Keyboard input handling systems (GATEWAY ONLY)

mod navigation;
mod editing;
mod selection;
mod clipboard;
mod undo_redo;
mod special;
mod handler;

// Re-export main keyboard handling function
pub use handler::handle_keyboard_input;
pub use special::handle_tab_navigation;
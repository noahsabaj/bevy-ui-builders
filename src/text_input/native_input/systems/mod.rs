//! System implementations for native text input (GATEWAY ONLY)
//!
//! This module serves as the gateway for all text input systems,
//! organizing them into logical submodules for better maintainability.

mod initialization;
mod io;
mod rendering;
mod cursor;
mod focus_visual;

// Re-export all public systems
pub use initialization::init_text_input;
pub use io::keyboard::{handle_keyboard_input, handle_tab_navigation};
pub use io::mouse::{handle_mouse_input, handle_mouse_drag, handle_click_outside};
pub use rendering::{render_text, render_selection};
pub use cursor::update_cursor_blink;
pub use focus_visual::update_focus_visual;
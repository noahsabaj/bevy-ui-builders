//! Mouse input handling systems (GATEWAY ONLY)

mod click;
mod drag;
mod selection;
mod outside;

// Re-export main mouse handling functions
pub use click::handle_mouse_input;
pub use drag::handle_mouse_drag;
pub use outside::handle_click_outside;
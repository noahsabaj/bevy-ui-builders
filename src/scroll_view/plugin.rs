//! ScrollView plugin for handling mouse wheel scrolling and visual feedback

use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin that provides scrolling functionality
define_plugin!(ScrollViewPlugin {
    update: [
        handle_mouse_wheel_scroll,         // Mouse wheel scrolling
        update_scrollbar_visibility,       // Scrollbar auto-hide/fade
        handle_scrollbar_thumb_drag,       // Drag scrollbar thumb to scroll (BEFORE position update)
        handle_keyboard_scroll,            // Keyboard navigation (Page/Home/Arrows)
        handle_drag_scroll,                // Drag-to-scroll with mouse
        apply_kinetic_scrolling,           // Momentum/kinetic scrolling
        auto_scroll_to_focused_input,      // Auto-scroll to focused text inputs
        clamp_scroll_bounds,               // Clamp scroll position to valid range
        update_scrollbar_thumb_position,   // Update thumb position/size based on scroll (AFTER all scroll updates)
    ]
});

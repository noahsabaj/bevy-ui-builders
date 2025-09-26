//! Text input plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::native_input::*;
use super::systems::*;

// Plugin that provides the complete text input system
define_plugin!(TextInputPlugin {
    events: [
        TextInputSubmitEvent,
        TextInputChangeEvent
    ],
    custom_init: |app: &mut App| {
        app.add_observer(init_text_input);
    },
    update: [
        // Native input systems
        handle_keyboard_input,
        handle_tab_navigation,
        handle_click_outside,  // Must run BEFORE handle_mouse_input to avoid race condition
        handle_mouse_input,     // This sets focus on clicked inputs
        handle_mouse_drag,
        update_cursor_blink,
        update_focus_visual,    // Maintain focus border color
        render_text,
        render_selection,

        // Legacy focus management (will be updated)
        handle_text_input_focus,
        validate_text_input_changes,
        handle_clear_button_clicks
    ]
});
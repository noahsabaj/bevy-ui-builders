//! Text input plugin

use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin that provides the complete text input system
define_plugin!(TextInputPlugin {
    update: [
        handle_text_input_focus,
        handle_click_outside_unfocus,
        validate_text_input_changes,
        handle_clear_button_clicks
    ]
});
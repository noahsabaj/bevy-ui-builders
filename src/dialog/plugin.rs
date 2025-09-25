//! Dialog plugin

use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin that adds dialog interaction systems
define_plugin!(DialogPlugin {
    events: [DialogButtonEvent],
    update: [
        handle_dialog_escape,
        handle_dialog_overlay_click,
        handle_cancel_button,
        emit_dialog_button_events
    ]
});
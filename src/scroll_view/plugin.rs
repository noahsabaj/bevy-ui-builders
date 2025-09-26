//! ScrollView plugin for handling mouse wheel scrolling and visual feedback

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin that provides scrolling functionality
define_plugin!(ScrollViewPlugin {
    update: [
        handle_mouse_wheel_scroll,
        update_scrollbar_visuals,
        handle_scrollbar_interaction,
        smooth_scroll_animation,
        auto_scroll_to_focused_input,
        update_scroll_limits,
    ]
});
//! Button plugin for Bevy

use bevy::prelude::IntoScheduleConfigs;
use bevy_plugin_builder::define_plugin;
use super::systems::{handle_hover_scale, handle_hover_brightness, handle_button_interaction, animate_button_transitions};

// Plugin that adds button interaction systems
define_plugin!(ButtonPlugin {
    update: [
        (handle_button_interaction, animate_button_transitions).chain(), // Chain ensures interaction runs before animation
        handle_hover_scale,         // Legacy system for explicit hover scale
        handle_hover_brightness     // Legacy system for explicit brightness
    ]
});
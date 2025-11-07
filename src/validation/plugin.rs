//! Plugin for input validation

use bevy_plugin_builder::define_plugin;
use super::systems::*;

define_plugin!(ValidationPlugin {
    update: [
        validate_text_inputs,
    ]
});

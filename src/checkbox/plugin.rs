//! Plugin for checkbox functionality

use bevy_plugin_builder::define_plugin;
use super::systems::*;

define_plugin!(CheckboxPlugin {
    update: [
        handle_checkbox_toggle,
        update_checkbox_visuals,
    ]
});

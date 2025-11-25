//! Plugin for checkbox functionality
#![allow(missing_docs)]

use bevy_plugin_builder::define_plugin;
use super::systems::*;

/// Plugin for checkbox functionality
define_plugin!(CheckboxPlugin {
    update: [
        handle_checkbox_toggle,
        update_checkbox_visuals,
    ]
});

//! Plugin for dropdown functionality
//! Dropdown plugin
#![allow(missing_docs)]

use bevy_plugin_builder::define_plugin;
use super::systems::*;

/// Plugin for dropdown functionality
define_plugin!(DropdownPlugin {
    update: [
        handle_dropdown_button_clicks,
        handle_dropdown_option_clicks,
        close_dropdown_on_outside_click,
        update_dropdown_selection_highlights,
        update_dropdown_option_hover,
    ]
});

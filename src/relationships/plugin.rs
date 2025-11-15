//! Plugin for UI relationships systems

use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin to register relationship systems.
//
// This plugin adds systems that manage UI relationships,
// such as exclusive button groups and slider part updates.
define_plugin!(UIRelationshipsPlugin {
    update: [
        // NOTE: Button group exclusivity is now handled by ButtonPlugin's enforce_exclusive_button_groups
        // handle_exclusive_button_groups,  // REMOVED - conflicts with new selection system
        update_slider_parts,
    ]
});
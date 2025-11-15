//! Button plugin for Bevy

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::systems::{
    handle_hover_scale,
    handle_hover_brightness,
    handle_button_interaction,
    animate_button_transitions,
    auto_toggle_selectable_buttons,
    enforce_exclusive_button_groups,
    update_selection_appearance,
    apply_selection_colors_immediately,
};
use super::types::SelectionChanged;

// Plugin that adds button interaction systems
define_plugin!(ButtonPlugin {
    custom_init: |app: &mut App| {
        // Register selection changed message
        app.add_message::<SelectionChanged>();
    },
    update: [
        // Selection state management - CHAINED to ensure commands are applied!
        (
            // Step 1: Handle button clicks and modify Selected components
            (enforce_exclusive_button_groups, auto_toggle_selectable_buttons),
            // Step 2: Update target colors based on Selected/Active (needs Selected changes applied)
            update_selection_appearance,
            // Step 3: Apply colors to BackgroundColor/BorderColor
            apply_selection_colors_immediately,
        ).chain(),

        // Animation and interaction
        (handle_button_interaction, animate_button_transitions).chain(),

        // Legacy hover systems
        (handle_hover_scale, handle_hover_brightness),
    ]
});
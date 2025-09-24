//! Button plugin for Bevy

use bevy::prelude::*;
use super::systems::{handle_hover_scale, handle_hover_brightness, handle_button_interaction, animate_button_transitions};

/// Plugin that adds button interaction systems
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_button_interaction,  // Sets animation targets on interaction change
            animate_button_transitions, // Smoothly animates to targets
            handle_hover_scale,         // Legacy system for explicit hover scale
            handle_hover_brightness,    // Legacy system for explicit brightness
        ));
    }
}
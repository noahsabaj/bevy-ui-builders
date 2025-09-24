//! Hover effects system for UI elements

use bevy::prelude::*;

/// Plugin that adds hover effect systems for all UI elements
pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, _app: &mut App) {
        // Hover systems will be added by individual builder plugins
        // This is here for future centralized hover effects
    }
}
//! Slider plugin

use bevy::prelude::*;
use super::systems::{
    handle_slider_interaction,
    update_slider_visuals,
    handle_slider_button_clicks
};

/// Plugin that provides slider systems
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_slider_interaction,
                update_slider_visuals,
                handle_slider_button_clicks,
            ),
        );
    }
}
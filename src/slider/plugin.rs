//! Slider plugin

use bevy_plugin_builder::define_plugin;
use super::systems::{
    handle_slider_interaction,
    update_slider_visuals,
    handle_slider_button_clicks
};

// Plugin that provides slider systems
define_plugin!(SliderPlugin {
    update: [
        handle_slider_interaction,
        update_slider_visuals,
        handle_slider_button_clicks
    ]
});
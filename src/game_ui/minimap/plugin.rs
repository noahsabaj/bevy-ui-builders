//! Minimap plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;

define_plugin!(MinimapPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(MinimapSettings::default());
    }
});

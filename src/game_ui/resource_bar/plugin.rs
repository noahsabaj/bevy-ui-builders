//! Resource bar plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;
use super::systems::*;

define_plugin!(ResourceBarPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(ResourceBarSettings::default())
           .add_message::<ResourceBarChanged>();
    },
    update: [
        animate_resource_bar_fill,
        animate_damage_indicator,
    ]
});

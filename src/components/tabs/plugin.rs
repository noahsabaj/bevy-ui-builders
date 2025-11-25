//! Tab view plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::TabSelectedEvent;
use super::systems::*;

define_plugin!(TabsPlugin {
    custom_init: |app: &mut App| {
        app.add_message::<TabSelectedEvent>();
    },
    update: [
        handle_tab_clicks,
        update_tab_button_visuals,
        update_tab_content_visibility,
        handle_tab_hover,
    ]
});

//! Tooltip plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::TooltipSettings;
use super::systems::*;

define_plugin!(TooltipPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(TooltipSettings::default());
    },
    update: [
        track_tooltip_hover,
        update_tooltip_hover_time,
        show_tooltips,
        hide_tooltips,
        cleanup_orphaned_tooltips,
    ]
});

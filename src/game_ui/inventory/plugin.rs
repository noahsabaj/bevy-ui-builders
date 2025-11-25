//! Inventory plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;
use super::systems::*;

define_plugin!(InventoryPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(InventorySettings::default())
           .insert_resource(InventoryDragState::default())
           .add_message::<SlotClickEvent>()
           .add_message::<ItemDragStartEvent>()
           .add_message::<ItemDropEvent>();
    },
    update: [
        handle_slot_hover,
        handle_slot_clicks,
        handle_drag_start,
        handle_drop,
        cancel_drag_on_right_click,
    ]
});

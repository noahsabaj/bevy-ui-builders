//! Context menu plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;
use super::systems::*;

define_plugin!(ContextMenuPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(ContextMenuSettings::default())
           .insert_resource(OpenContextMenu::default())
           .add_message::<ContextMenuActionEvent>()
           .add_message::<ContextMenuCheckboxEvent>();
    },
    update: [
        detect_context_menu_trigger,
        handle_menu_item_hover,
        handle_menu_item_click,
        close_menu_on_outside_click,
        close_menu_on_escape,
    ]
});

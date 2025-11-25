//! Dialogue plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;
use super::systems::*;

define_plugin!(DialoguePlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(DialogueSettings::default())
           .add_message::<DialogueAdvanceEvent>()
           .add_message::<DialogueChoiceEvent>()
           .add_message::<DialogueTypingCompleteEvent>();
    },
    update: [
        update_typing_effect,
        handle_skip_typing,
        handle_choice_hover,
        handle_choice_clicks,
    ]
});

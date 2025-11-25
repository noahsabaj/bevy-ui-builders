//! Toast plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::types::*;
use super::systems::*;

define_plugin!(ToastPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(ToastSettings::default())
           .insert_resource(ToastQueue::default())
           .add_message::<ToastActionEvent>()
           .add_message::<DismissToastEvent>();
    },
    update: [
        ensure_toast_container,
        spawn_toasts,
        update_toast_timers,
        handle_toast_dismiss,
        handle_toast_action,
        handle_dismiss_events,
        despawn_exiting_toasts,
    ]
});

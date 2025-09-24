//! Dialog plugin

use bevy::prelude::*;
use super::systems::*;

/// Plugin that adds dialog interaction systems
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DialogButtonEvent>()
            .add_systems(
                Update,
                (
                    handle_dialog_escape,
                    handle_dialog_overlay_click,
                    handle_cancel_button,
                    emit_dialog_button_events,
                ),
            );
    }
}
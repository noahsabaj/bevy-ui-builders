//! Plugin for UI relationships systems

use bevy::prelude::*;
use super::systems::*;

/// Plugin to register relationship systems.
///
/// This plugin adds systems that manage UI relationships,
/// such as exclusive button groups and slider part updates.
pub struct UIRelationshipsPlugin;

impl Plugin for UIRelationshipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_exclusive_button_groups,
                update_slider_parts,
            ),
        );
    }
}
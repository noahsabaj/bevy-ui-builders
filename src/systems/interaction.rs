//! Generic interaction handling for UI elements

use bevy::prelude::*;

/// Handle generic UI interactions
pub fn handle_interactions(
    interactions: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, entity) in &interactions {
        match interaction {
            Interaction::Pressed => {
                // Button pressed
            }
            Interaction::Hovered => {
                // Button hovered
            }
            Interaction::None => {}
        }
    }
}
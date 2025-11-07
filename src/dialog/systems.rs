//! Dialog interaction systems

use bevy::prelude::*;
use super::types::{DialogOverlay, CancelButton};

/// System to handle ESC key for dismissible dialogs
pub fn handle_dialog_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    overlay_query: Query<(Entity, &DialogOverlay)>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        for (entity, overlay) in &overlay_query {
            if overlay.dismissible {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// System to handle clicking outside dismissible dialogs
pub fn handle_dialog_overlay_click(
    mut commands: Commands,
    overlay_query: Query<(Entity, &DialogOverlay, &Interaction), Changed<Interaction>>,
) {
    for (entity, overlay, interaction) in &overlay_query {
        if *interaction == Interaction::Pressed && overlay.dismissible {
            commands.entity(entity).despawn();
        }
    }
}

/// System to handle cancel button clicks
pub fn handle_cancel_button(
    mut commands: Commands,
    button_query: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    overlay_query: Query<Entity, With<DialogOverlay>>,
) {
    for interaction in &button_query {
        if *interaction == Interaction::Pressed {
            // Despawn all dialog overlays when cancel is pressed
            for entity in &overlay_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Event fired when a dialog button is clicked
#[derive(Message)]
pub struct DialogButtonEvent {
    pub dialog_type: super::types::DialogType,
    pub button_marker: String,
}

/// System to emit events for dialog button clicks
pub fn emit_dialog_button_events(
    mut events: MessageWriter<DialogButtonEvent>,
    confirm_query: Query<&Interaction, (Changed<Interaction>, With<super::types::ConfirmButton>)>,
    save_query: Query<&Interaction, (Changed<Interaction>, With<super::types::SaveButton>)>,
    discard_query: Query<&Interaction, (Changed<Interaction>, With<super::types::DiscardButton>)>,
    ok_query: Query<&Interaction, (Changed<Interaction>, With<super::types::OkButton>)>,
    yes_query: Query<&Interaction, (Changed<Interaction>, With<super::types::YesButton>)>,
    no_query: Query<&Interaction, (Changed<Interaction>, With<super::types::NoButton>)>,
    overlay_query: Query<&DialogOverlay>,
) {
    // Check each button type
    for interaction in &confirm_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "confirm".to_string(),
                });
            }
        }
    }

    for interaction in &save_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "save".to_string(),
                });
            }
        }
    }

    for interaction in &discard_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "discard".to_string(),
                });
            }
        }
    }

    for interaction in &ok_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "ok".to_string(),
                });
            }
        }
    }

    for interaction in &yes_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "yes".to_string(),
                });
            }
        }
    }

    for interaction in &no_query {
        if *interaction == Interaction::Pressed {
            if let Ok(overlay) = overlay_query.single() {
                events.write(DialogButtonEvent {
                    dialog_type: overlay.dialog_type,
                    button_marker: "no".to_string(),
                });
            }
        }
    }
}
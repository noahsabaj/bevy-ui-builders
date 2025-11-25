//! Example demonstrating how to add custom marker components to dialog buttons

use bevy::prelude::*;
use bevy_ui_builders::prelude::*;
use bevy_ui_builders::*;

// Custom marker components for your game logic
#[derive(Component)]
struct DeleteConfirmButton;

#[derive(Component)]
struct DeleteCancelButton;

#[derive(Component)]
struct CustomActionButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_dialog_buttons,
            handle_custom_dialog_buttons,
            spawn_dialogs_on_key_press,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Show instructions
    commands.spawn((
        Text::new("Press 1-4 to spawn different dialog examples:\n\n\
                  1 - Standard dialog (uses bevy-ui-builders markers)\n\
                  2 - Dialog with custom markers (build_with_buttons)\n\
                  3 - Dialog with single custom marker (build_and_mark)\n\
                  4 - Exit confirmation with custom logic\n\n\
                  ESC - Close dialogs"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

fn spawn_dialogs_on_key_press(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    existing_dialogs: Query<Entity, With<DialogOverlay>>,
) {
    // Close dialogs on ESC
    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in &existing_dialogs {
            commands.entity(entity).despawn();
        }
    }

    // Example 1: Standard dialog using bevy-ui-builders' built-in markers
    if keyboard.just_pressed(KeyCode::Digit1) {
        DialogBuilder::new(DialogType::Custom)
            .title("Standard Dialog")
            .body("This dialog uses the standard ConfirmButton and CancelButton markers from bevy-ui-builders")
            .confirm_button("Confirm")
            .cancel_button("Cancel")
            .build(&mut commands);
    }

    // Example 2: Dialog with custom markers using build_with_buttons
    if keyboard.just_pressed(KeyCode::Digit2) {
        let (dialog, buttons) = DialogBuilder::new(DialogType::Custom)
            .title("Delete Item?")
            .body("This action cannot be undone. Custom markers will be added to buttons.")
            .danger_button("Delete")
            .cancel_button("Cancel")
            .build_with_buttons(&mut commands);

        // Add custom markers to specific buttons
        if let Some(delete_btn) = buttons.get(&DialogButtonMarker::Confirm) {
            commands.entity(*delete_btn).insert(DeleteConfirmButton);
        }
        if let Some(cancel_btn) = buttons.get(&DialogButtonMarker::Cancel) {
            commands.entity(*cancel_btn).insert(DeleteCancelButton);
        }

        info!("Spawned dialog {} with custom button markers", dialog);
    }

    // Example 3: Dialog with single custom marker using build_and_mark helper
    if keyboard.just_pressed(KeyCode::Digit3) {
        DialogBuilder::new(DialogType::Custom)
            .title("Quick Action")
            .body("Using build_and_mark to add a single custom marker")
            .confirm_button("Do It!")
            .cancel_button("Cancel")
            .build_and_mark(&mut commands, DialogButtonMarker::Confirm, CustomActionButton);
    }

    // Example 4: Complex dialog with multiple custom buttons
    if keyboard.just_pressed(KeyCode::Digit4) {
        let (dialog, buttons) = DialogBuilder::new(DialogType::ExitConfirmation)
            .title("Exit Game?")
            .body("Save your progress before exiting?")
            .save_button("Save & Exit")
            .danger_button("Exit Without Saving")
            .cancel_button("Cancel")
            .build_with_buttons(&mut commands);

        // Add custom markers to all buttons for game-specific logic
        for (marker, entity) in buttons {
            match marker {
                DialogButtonMarker::Save => {
                    commands.entity(entity).insert((
                        DeleteConfirmButton, // Reusing for demo
                        Name::new("SaveAndExitButton"),
                    ));
                }
                DialogButtonMarker::Confirm => {
                    commands.entity(entity).insert((
                        DeleteConfirmButton,
                        Name::new("ExitWithoutSavingButton"),
                    ));
                }
                DialogButtonMarker::Cancel => {
                    commands.entity(entity).insert((
                        DeleteCancelButton,
                        Name::new("CancelExitButton"),
                    ));
                }
                _ => {}
            }
        }
    }
}

// Handle standard dialog buttons using bevy-ui-builders markers
fn handle_dialog_buttons(
    mut commands: Commands,
    interactions: Query<
        (&Interaction, AnyOf<(&ConfirmButton, &CancelButton, &SaveButton)>),
        Changed<Interaction>,
    >,
    dialogs: Query<Entity, With<DialogOverlay>>,
) {
    for (interaction, buttons) in &interactions {
        if *interaction == Interaction::Pressed {
            let (confirm, cancel, save) = buttons;

            if confirm.is_some() {
                info!("Standard Confirm button pressed!");
            } else if cancel.is_some() {
                info!("Standard Cancel button pressed!");
            } else if save.is_some() {
                info!("Standard Save button pressed!");
                // Perform save logic here
                info!("Saving game...");
            }

            // Close all dialogs (Bevy 0.16 despawns recursively)
            for dialog in &dialogs {
                commands.entity(dialog).despawn();
            }
        }
    }
}

// Handle custom dialog buttons using your own marker components
fn handle_custom_dialog_buttons(
    mut commands: Commands,
    delete_interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<DeleteConfirmButton>),
    >,
    cancel_interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<DeleteCancelButton>),
    >,
    custom_interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<CustomActionButton>),
    >,
    dialogs: Query<Entity, With<DialogOverlay>>,
) {
    let mut should_close = false;

    // Handle delete confirmation
    for interaction in &delete_interactions {
        if *interaction == Interaction::Pressed {
            info!("Custom Delete Confirm button pressed! Performing delete...");
            // Perform your delete logic here
            should_close = true;
        }
    }

    // Handle delete cancel
    for interaction in &cancel_interactions {
        if *interaction == Interaction::Pressed {
            info!("Custom Delete Cancel button pressed!");
            should_close = true;
        }
    }

    // Handle custom action
    for interaction in &custom_interactions {
        if *interaction == Interaction::Pressed {
            info!("Custom Action button pressed! Doing the thing...");
            // Perform your custom action here
            should_close = true;
        }
    }

    // Close all dialogs if any button was pressed
    if should_close {
        for dialog in &dialogs {
            commands.entity(dialog).despawn();
        }
    }
}
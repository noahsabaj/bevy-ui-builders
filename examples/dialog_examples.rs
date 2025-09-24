//! Dialog Examples - Demonstrates various dialog types and configurations
//!
//! Run with: cargo run --example dialog_examples --features dialog

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::dialog::presets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, dialog_button_handler))
        .run();
}

#[derive(Component)]
struct DialogTrigger {
    dialog_type: DemoDialogType,
}

#[derive(Clone, Copy)]
enum DemoDialogType {
    ExitConfirmation,
    UnsavedChanges,
    Error,
    Info,
    Warning,
    Success,
    Custom,
    NonDismissible,
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root node with buttons to trigger different dialogs
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(40.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Dialog Examples"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Instructions
            parent.spawn((
                Text::new("Click any button to show a dialog"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Dialog Type Buttons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|column| {
                    // Row 1: Basic dialog types
                    column
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|row| {
                            create_dialog_button(
                                row,
                                "Exit Confirmation",
                                ButtonStyle::Primary,
                                DemoDialogType::ExitConfirmation,
                            );

                            create_dialog_button(
                                row,
                                "Unsaved Changes",
                                ButtonStyle::Warning,
                                DemoDialogType::UnsavedChanges,
                            );

                            create_dialog_button(
                                row,
                                "Error Dialog",
                                ButtonStyle::Danger,
                                DemoDialogType::Error,
                            );
                        });

                    // Row 2: Message dialogs
                    column
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|row| {
                            create_dialog_button(
                                row,
                                "Info Dialog",
                                ButtonStyle::Secondary,
                                DemoDialogType::Info,
                            );

                            create_dialog_button(
                                row,
                                "Warning Dialog",
                                ButtonStyle::Warning,
                                DemoDialogType::Warning,
                            );

                            create_dialog_button(
                                row,
                                "Success Dialog",
                                ButtonStyle::Success,
                                DemoDialogType::Success,
                            );
                        });

                    // Row 3: Special dialogs
                    column
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|row| {
                            create_dialog_button(
                                row,
                                "Custom Dialog",
                                ButtonStyle::Primary,
                                DemoDialogType::Custom,
                            );

                            create_dialog_button(
                                row,
                                "Non-Dismissible",
                                ButtonStyle::Danger,
                                DemoDialogType::NonDismissible,
                            );
                        });
                });
        });
}

fn create_dialog_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    style: ButtonStyle,
    dialog_type: DemoDialogType,
) {
    let button = ButtonBuilder::new(text)
        .style(style)
        .size(ButtonSize::Large)
        .build(parent);

    parent.commands().entity(button).insert(DialogTrigger { dialog_type });
}

fn button_system(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &DialogTrigger),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, trigger) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match trigger.dialog_type {
                DemoDialogType::ExitConfirmation => {
                    // Using the preset
                    presets::exit_confirmation(&mut commands);
                    info!("Showing exit confirmation dialog");
                }
                DemoDialogType::UnsavedChanges => {
                    // Using the preset
                    presets::unsaved_changes(&mut commands);
                    info!("Showing unsaved changes dialog");
                }
                DemoDialogType::Error => {
                    // Using the preset with custom message
                    presets::error(&mut commands, "Failed to load the requested file. Please check the file path and try again.");
                    info!("Showing error dialog");
                }
                DemoDialogType::Info => {
                    // Custom info dialog
                    DialogBuilder::new(DialogType::Info)
                        .title("Information")
                        .body("This is an informational message. It provides helpful details about the current operation.")
                        .ok_button()
                        .build(&mut commands);
                    info!("Showing info dialog");
                }
                DemoDialogType::Warning => {
                    // Custom warning dialog
                    DialogBuilder::new(DialogType::Warning)
                        .title("Warning")
                        .body("This action may have unintended consequences. Are you sure you want to proceed?")
                        .confirm_button("Continue")
                        .cancel_button("Cancel")
                        .build(&mut commands);
                    info!("Showing warning dialog");
                }
                DemoDialogType::Success => {
                    // Custom success dialog
                    DialogBuilder::new(DialogType::Success)
                        .title("Success!")
                        .body("The operation completed successfully. Your changes have been saved.")
                        .ok_button()
                        .z_index(1000)
                        .build(&mut commands);
                    info!("Showing success dialog");
                }
                DemoDialogType::Custom => {
                    // Fully custom dialog with multiple buttons
                    DialogBuilder::new(DialogType::Custom)
                        .title("Custom Dialog")
                        .body("This is a custom dialog with multiple action buttons. You can configure it however you like.")
                        .confirm_button("Save")
                        .cancel_button("Cancel")
                        .danger_button("Delete")
                        .dismissible(true)
                        .build(&mut commands);
                    info!("Showing custom dialog");
                }
                DemoDialogType::NonDismissible => {
                    // Non-dismissible dialog (can't click outside to close)
                    DialogBuilder::new(DialogType::Error)
                        .title("Critical Error")
                        .body("A critical error has occurred. You must acknowledge this message before continuing.")
                        .danger_button("Acknowledge")
                        .dismissible(false)
                        .z_index(2000)
                        .build(&mut commands);
                    info!("Showing non-dismissible dialog");
                }
            }
        }
    }
}

// System to handle dialog button events
fn dialog_button_handler(
    mut commands: Commands,
    mut events: EventReader<DialogButtonEvent>,
    dialog_query: Query<Entity, With<DialogOverlay>>,
) {
    for event in events.read() {
        info!("Dialog button clicked: {} from {:?}", event.button_marker, event.dialog_type);

        match event.button_marker.as_str() {
            "Confirm" | "Save" | "Continue" => {
                info!("Dialog confirmed");
            }
            "Cancel" => {
                info!("Dialog cancelled");
            }
            "Delete" | "Acknowledge" => {
                info!("Danger action selected");
            }
            "OK" => {
                info!("Dialog acknowledged");
            }
            _ => {}
        }

        // Close the dialog
        for dialog_entity in &dialog_query {
            commands.entity(dialog_entity).despawn_recursive();
        }
    }
}
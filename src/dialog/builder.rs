//! DialogBuilder implementation

use bevy::prelude::*;
use crate::button::{ButtonBuilder, ButtonSize};
use crate::styles::{colors, dimensions, ButtonStyle};
use super::types::*;

/// Builder for creating dialogs
pub struct DialogBuilder {
    title: String,
    body: String,
    dialog_type: DialogType,
    width: Val,
    min_width: Val,
    max_width: Val,
    height: Val,
    min_height: Val,
    max_height: Val,
    buttons: Vec<DialogButton>,
    dismissible: bool,
    z_index: i32,
}

impl DialogBuilder {
    /// Create a new dialog builder
    pub fn new(dialog_type: DialogType) -> Self {
        Self {
            title: String::new(),
            body: String::new(),
            dialog_type,
            width: Val::Px(dimensions::DIALOG_WIDTH_MEDIUM),
            min_width: Val::Auto,
            max_width: Val::Auto,
            height: Val::Auto,
            min_height: Val::Auto,
            max_height: Val::Auto,
            buttons: Vec::new(),
            dismissible: true,
            z_index: dimensions::Z_INDEX_MODAL,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog body text
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Set the dialog width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, min_width: Val) -> Self {
        self.min_width = min_width;
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.max_width = max_width;
        self
    }

    /// Set the dialog height
    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    /// Set minimum height
    pub fn min_height(mut self, min_height: Val) -> Self {
        self.min_height = min_height;
        self
    }

    /// Set maximum height
    pub fn max_height(mut self, max_height: Val) -> Self {
        self.max_height = max_height;
        self
    }

    /// Set whether the dialog can be dismissed by clicking outside
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set the z-index for layering
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    /// Add a confirm button
    pub fn confirm_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Primary,
            marker: DialogButtonMarker::Confirm,
        });
        self
    }

    /// Add a cancel button
    pub fn cancel_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Secondary,
            marker: DialogButtonMarker::Cancel,
        });
        self
    }

    /// Add a danger button
    pub fn danger_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Danger,
            marker: DialogButtonMarker::Confirm,
        });
        self
    }

    /// Add a save button
    pub fn save_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Success,
            marker: DialogButtonMarker::Save,
        });
        self
    }

    /// Add a discard button
    pub fn discard_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Warning,
            marker: DialogButtonMarker::Discard,
        });
        self
    }

    /// Add an OK button
    pub fn ok_button(mut self) -> Self {
        self.buttons.push(DialogButton {
            text: "OK".to_string(),
            style: ButtonStyle::Primary,
            marker: DialogButtonMarker::Ok,
        });
        self
    }

    /// Add Yes/No buttons
    pub fn yes_no_buttons(mut self) -> Self {
        self.buttons.push(DialogButton {
            text: "Yes".to_string(),
            style: ButtonStyle::Primary,
            marker: DialogButtonMarker::Yes,
        });
        self.buttons.push(DialogButton {
            text: "No".to_string(),
            style: ButtonStyle::Secondary,
            marker: DialogButtonMarker::No,
        });
        self
    }

    /// Add a custom button
    pub fn custom_button(
        mut self,
        text: impl Into<String>,
        style: ButtonStyle,
        marker: DialogButtonMarker,
    ) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style,
            marker,
        });
        self
    }

    /// Build the dialog entity
    pub fn build(self, commands: &mut Commands) -> Entity {
        // Create overlay that blocks clicks
        let overlay_entity = commands
            .spawn((
                Button, // Block clicks to elements behind
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::OVERLAY_BACKDROP),
                DialogOverlay {
                    dialog_type: self.dialog_type,
                    dismissible: self.dismissible,
                },
                ZIndex(self.z_index),
            ))
            .id();

        // Add type-specific marker
        match self.dialog_type {
            DialogType::ExitConfirmation => {
                commands.entity(overlay_entity).insert(ExitConfirmationDialog);
            }
            DialogType::UnsavedChanges => {
                commands.entity(overlay_entity).insert(UnsavedChangesDialog);
            }
            DialogType::Resolution => {
                commands.entity(overlay_entity).insert(ResolutionDialog);
            }
            DialogType::Error => {
                commands.entity(overlay_entity).insert(ErrorDialog);
            }
            DialogType::Info => {
                commands.entity(overlay_entity).insert(InfoDialog);
            }
            DialogType::Warning => {
                commands.entity(overlay_entity).insert(WarningDialog);
            }
            DialogType::Success => {
                commands.entity(overlay_entity).insert(SuccessDialog);
            }
            DialogType::Custom => {}
        }

        // Create container
        let container_entity = commands
            .spawn((
                Node {
                    width: self.width,
                    height: self.height,
                    min_width: self.min_width,
                    min_height: self.min_height,
                    max_width: self.max_width,
                    max_height: self.max_height,
                    padding: UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_SECONDARY),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_LARGE)),
                DialogContainer {
                    dialog_type: self.dialog_type,
                },
                ZIndex(self.z_index + 50),
            ))
            .id();

        commands.entity(container_entity).with_children(|parent| {
            // Title
            if !self.title.is_empty() {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            margin: UiRect::bottom(Val::Px(dimensions::SPACING_LARGE)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|title_parent| {
                        title_parent.spawn((
                            Text::new(self.title.clone()),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_HEADING,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            DialogTitle,
                        ));
                    });
            }

            // Body
            if !self.body.is_empty() {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            margin: UiRect::bottom(Val::Px(dimensions::SPACING_LARGE)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|body_parent| {
                        body_parent.spawn((
                            Text::new(self.body.clone()),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_MEDIUM,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            DialogBody,
                        ));
                    });
            }

            // Buttons
            if !self.buttons.is_empty() {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(dimensions::SPACING_MEDIUM),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        DialogButtonRow,
                    ))
                    .with_children(|button_row| {
                        for button in self.buttons {
                            let button_entity = ButtonBuilder::new(button.text)
                                .style(button.style)
                                .size(ButtonSize::Medium)
                                .build(button_row);

                            // Add marker based on type
                            match button.marker {
                                DialogButtonMarker::Confirm => {
                                    button_row.commands().entity(button_entity).insert(ConfirmButton);
                                }
                                DialogButtonMarker::Cancel => {
                                    button_row.commands().entity(button_entity).insert(CancelButton);
                                }
                                DialogButtonMarker::Save => {
                                    button_row.commands().entity(button_entity).insert(SaveButton);
                                }
                                DialogButtonMarker::Discard => {
                                    button_row.commands().entity(button_entity).insert(DiscardButton);
                                }
                                DialogButtonMarker::Ok => {
                                    button_row.commands().entity(button_entity).insert(OkButton);
                                }
                                DialogButtonMarker::Yes => {
                                    button_row.commands().entity(button_entity).insert(YesButton);
                                }
                                DialogButtonMarker::No => {
                                    button_row.commands().entity(button_entity).insert(NoButton);
                                }
                                DialogButtonMarker::Custom(_) => {
                                    // Custom markers can be added by the user after creation
                                }
                            }
                        }
                    });
            }
        });

        // Add container as child of overlay
        commands.entity(overlay_entity).add_child(container_entity);

        overlay_entity
    }
}

/// Convenience functions for common dialogs
pub mod presets {
    use super::*;

    /// Create an exit confirmation dialog
    pub fn exit_confirmation(commands: &mut Commands) -> Entity {
        DialogBuilder::new(DialogType::ExitConfirmation)
            .title("Exit Application")
            .body("Are you sure you want to exit?")
            .danger_button("Exit")
            .cancel_button("Cancel")
            .build(commands)
    }

    /// Create an unsaved changes dialog
    pub fn unsaved_changes(commands: &mut Commands) -> Entity {
        DialogBuilder::new(DialogType::UnsavedChanges)
            .title("Unsaved Changes")
            .body("You have unsaved changes. What would you like to do?")
            .save_button("Save")
            .discard_button("Discard")
            .cancel_button("Cancel")
            .build(commands)
    }

    /// Create an error dialog
    pub fn error(commands: &mut Commands, message: impl Into<String>) -> Entity {
        DialogBuilder::new(DialogType::Error)
            .title("Error")
            .body(message)
            .ok_button()
            .build(commands)
    }

    /// Create an info dialog
    pub fn info(commands: &mut Commands, title: impl Into<String>, message: impl Into<String>) -> Entity {
        DialogBuilder::new(DialogType::Info)
            .title(title)
            .body(message)
            .ok_button()
            .build(commands)
    }

    /// Create a warning dialog
    pub fn warning(commands: &mut Commands, message: impl Into<String>) -> Entity {
        DialogBuilder::new(DialogType::Warning)
            .title("Warning")
            .body(message)
            .ok_button()
            .build(commands)
    }

    /// Create a success dialog
    pub fn success(commands: &mut Commands, message: impl Into<String>) -> Entity {
        DialogBuilder::new(DialogType::Success)
            .title("Success")
            .body(message)
            .ok_button()
            .build(commands)
    }

    /// Create a confirmation dialog
    pub fn confirm(commands: &mut Commands, title: impl Into<String>, message: impl Into<String>) -> Entity {
        DialogBuilder::new(DialogType::Custom)
            .title(title)
            .body(message)
            .yes_no_buttons()
            .build(commands)
    }
}
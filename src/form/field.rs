//! Form field spawning helper

use bevy::prelude::*;
use crate::styles::{colors, dimensions};
use super::types::{FormField, FieldType, FormFieldMarker};

/// Helper function to spawn a form field
pub fn spawn_form_field(parent: &mut ChildSpawnerCommands, field: &FormField) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|field_container| {
            // Label
            field_container.spawn((
                Text::new(&field.label),
                TextFont {
                    font_size: dimensions::FONT_SIZE_MEDIUM,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));

            // Field input based on type
            match &field.field_type {
                FieldType::Text | FieldType::Password | FieldType::Email => {
                    // Text input placeholder
                    field_container.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(dimensions::INPUT_HEIGHT),
                            padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                            border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_TERTIARY),
                        BorderColor(colors::BORDER_DEFAULT),
                        BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_SMALL)),
                        FormFieldMarker {
                            field_name: field.name.clone(),
                            field_type: field.field_type.clone(),
                        },
                    ));
                }
                FieldType::Checkbox => {
                    // Checkbox placeholder
                    field_container.spawn((
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_TERTIARY),
                        BorderColor(colors::BORDER_DEFAULT),
                        BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_SMALL)),
                        FormFieldMarker {
                            field_name: field.name.clone(),
                            field_type: field.field_type.clone(),
                        },
                    ));
                }
                _ => {
                    // Placeholder for other field types
                    field_container.spawn((
                        Text::new(format!("[{:?} field]", field.field_type)),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                    ));
                }
            }

            // Help text if provided
            if let Some(help_text) = &field.help_text {
                field_container.spawn((
                    Text::new(help_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
            }
        });
}
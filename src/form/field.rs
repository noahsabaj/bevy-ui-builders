//! Form field spawning helper

use bevy::prelude::*;
use crate::styles::{colors, dimensions};
use super::types::{FormField, FieldType, FormFieldMarker};

#[cfg(feature = "text_input")]
use crate::text_input::TextInputBuilder;

#[cfg(feature = "slider")]
use crate::slider::SliderBuilder;

#[cfg(feature = "checkbox")]
use crate::checkbox::CheckboxBuilder;

#[cfg(feature = "number_input")]
use crate::number_input::NumberInputBuilder;

#[cfg(feature = "dropdown")]
use crate::dropdown::DropdownBuilder;

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
            // Label (only show for non-checkbox fields, checkbox has its own label)
            if !matches!(field.field_type, FieldType::Checkbox) {
                field_container.spawn((
                    Text::new(&field.label),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_MEDIUM,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            }

            // Field input based on type
            match &field.field_type {
                #[cfg(feature = "text_input")]
                FieldType::Text => {
                    let mut builder = TextInputBuilder::new()
                        .with_width(Val::Percent(100.0));

                    if let Some(placeholder) = &field.placeholder {
                        builder = builder.with_placeholder(placeholder);
                    }

                    if let Some(default_value) = &field.default_value {
                        builder = builder.with_value(default_value);
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "text_input")]
                FieldType::Password => {
                    let mut builder = TextInputBuilder::new()
                        .with_width(Val::Percent(100.0));
                    // TODO: Add password masking visualization

                    if let Some(placeholder) = &field.placeholder {
                        builder = builder.with_placeholder(placeholder);
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "text_input")]
                FieldType::Email => {
                    // Use regex pattern for email validation if desired
                    let mut builder = TextInputBuilder::new()
                        .with_width(Val::Percent(100.0));

                    if let Some(placeholder) = &field.placeholder {
                        builder = builder.with_placeholder(placeholder);
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "number_input")]
                FieldType::Number { min, max } => {
                    let mut builder = NumberInputBuilder::new()
                        .width(Val::Percent(100.0));

                    if let Some(min_val) = min {
                        builder = builder.min(*min_val);
                    }

                    if let Some(max_val) = max {
                        builder = builder.max(*max_val);
                    }

                    if let Some(placeholder) = &field.placeholder {
                        builder = builder.with_placeholder(placeholder);
                    }

                    if let Some(default_value) = &field.default_value {
                        if let Ok(value) = default_value.parse::<f32>() {
                            builder = builder.default_value(value);
                        }
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "slider")]
                FieldType::Slider { min, max, step } => {
                    let default_value = field.default_value
                        .as_ref()
                        .and_then(|s| s.parse::<f32>().ok())
                        .unwrap_or(*min);

                    let mut builder = SliderBuilder::new(*min..*max)
                        .value(default_value)
                        .width(Val::Percent(100.0));

                    if let Some(step_val) = step {
                        builder = builder.step(*step_val);
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "checkbox")]
                FieldType::Checkbox => {
                    let checked = field.default_value
                        .as_ref()
                        .map(|s| s == "true" || s == "1")
                        .unwrap_or(false);

                    let entity = CheckboxBuilder::new()
                        .checked(checked)
                        .with_label(&field.label)
                        .build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(feature = "dropdown")]
                FieldType::Dropdown { options } => {
                    let mut builder = DropdownBuilder::new(options.clone())
                        .width(Val::Percent(100.0));

                    if let Some(placeholder) = &field.placeholder {
                        builder = builder.placeholder(placeholder);
                    }

                    // Parse default value as index if provided
                    if let Some(default_value) = &field.default_value {
                        if let Ok(index) = default_value.parse::<usize>() {
                            builder = builder.selected_index(Some(index));
                        }
                    }

                    let entity = builder.build(field_container);

                    field_container.commands().entity(entity).insert(FormFieldMarker {
                        field_name: field.name.clone(),
                        field_type: field.field_type.clone(),
                    });
                }

                #[cfg(not(feature = "dropdown"))]
                FieldType::Dropdown { options } => {
                    // Placeholder when dropdown feature is not enabled
                    field_container.spawn((
                        Text::new(format!("[Dropdown with {} options]", options.len())),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
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
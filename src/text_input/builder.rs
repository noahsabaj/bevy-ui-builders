//! TextInputBuilder implementation

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};
use crate::button::{ButtonBuilder, ButtonSize};
use crate::styles::{colors, ButtonStyle};
use super::types::*;

/// Builder for creating text inputs with managed focus
#[derive(Clone)]
pub struct TextInputBuilder {
    value: String,
    placeholder: Option<String>,
    font_size: f32,
    width: Val,
    height: Val,
    padding: UiRect,
    focus_type: TextInputFocus,
    inactive: bool,
    retain_on_submit: bool,
    filter: Option<TextInputFilter>,
    show_clear_button: bool,
}

/// Builder with a single marker component
pub struct TextInputBuilderWithMarker<M: Component> {
    builder: TextInputBuilder,
    marker: M,
}

/// Builder with two marker components
pub struct TextInputBuilderWithTwoMarkers<M: Component, N: Component> {
    builder: TextInputBuilder,
    marker1: M,
    marker2: N,
}

// Helper function to build text input with common components
fn build_text_input_with_extras<M>(
    parent: &mut ChildSpawnerCommands,
    builder: TextInputBuilder,
    extras: impl FnOnce(&mut EntityCommands) -> M,
) -> Entity {
    // If we need a clear button, create a container
    if builder.show_clear_button {
        let container_id = parent
            .spawn((
                Node {
                    width: builder.width,
                    height: builder.height,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .id();

        let mut text_input_id = None;

        parent
            .commands()
            .entity(container_id)
            .with_children(|container| {
                let mut entity_commands = container.spawn((
                    // Node components for layout
                    Node {
                        flex_grow: 1.0, // Take remaining space
                        height: Val::Percent(100.0),
                        padding: builder.padding,
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_LIGHT),
                    BorderColor(colors::BORDER_DEFAULT),
                    BorderRadius::all(Val::Px(5.0)),
                    TextInput,
                    TextInputValue(
                        if builder.value.is_empty() {
                            builder.placeholder.clone().unwrap_or_default()
                        } else {
                            builder.value.clone()
                        },
                    ),
                    TextInputTextFont(TextFont {
                        font_size: builder.font_size,
                        ..default()
                    }),
                    TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                    TextInputSettings {
                        retain_on_submit: builder.retain_on_submit,
                        ..default()
                    },
                    // Focus management
                    builder.focus_type.clone(),
                    // Make it a button so it can be clicked
                    Button,
                ));

                // Add extras from callback
                extras(&mut entity_commands);

                // Add inactive state if requested
                if builder.inactive {
                    entity_commands.insert(TextInputInactive(true));
                }

                // Add filter if specified
                if let Some(filter) = builder.filter.clone() {
                    entity_commands.insert(filter);
                }

                text_input_id = Some(entity_commands.id());

                // Add clear button
                let clear_button = ButtonBuilder::new("×")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(container);

                // Add component to track which text input this button clears
                if let Some(input_id) = text_input_id {
                    container
                        .commands()
                        .entity(clear_button)
                        .insert(ClearButtonTarget(input_id));
                }
            });

        container_id
    } else {
        // No clear button, build normally
        let mut entity_commands = parent.spawn((
            // Node components for layout
            Node {
                width: builder.width,
                height: builder.height,
                padding: builder.padding,
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
            TextInput,
            TextInputValue(
                if builder.value.is_empty() {
                    builder.placeholder.unwrap_or_default()
                } else {
                    builder.value
                },
            ),
            TextInputTextFont(TextFont {
                font_size: builder.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            TextInputSettings {
                retain_on_submit: builder.retain_on_submit,
                ..default()
            },
            // Focus management
            builder.focus_type.clone(),
            // Make it a button so it can be clicked
            Button,
        ));

        // Add extras from callback
        extras(&mut entity_commands);

        // Add inactive state if requested
        if builder.inactive {
            entity_commands.insert(TextInputInactive(true));
        }

        // Add filter if specified
        if let Some(filter) = builder.filter.clone() {
            entity_commands.insert(filter);
        }

        entity_commands.id()
    }
}

impl TextInputBuilder {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            font_size: 16.0,
            width: Val::Px(300.0),
            height: Val::Px(40.0),
            padding: UiRect::all(Val::Px(10.0)),
            focus_type: TextInputFocus::Independent,
            inactive: false,
            retain_on_submit: true,
            filter: None,
            show_clear_button: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set placeholder text (currently just sets initial value)
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    /// Set padding
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Make this input part of an exclusive focus group
    pub fn with_focus_group(mut self, group: FocusGroupId) -> Self {
        self.focus_type = TextInputFocus::ExclusiveGroup(group);
        self
    }

    /// Make this input independent (doesn't affect other inputs)
    pub fn independent(mut self) -> Self {
        self.focus_type = TextInputFocus::Independent;
        self
    }

    /// Start with the input inactive (not focused)
    pub fn inactive(mut self) -> Self {
        self.inactive = true;
        self
    }

    /// Set whether to retain text on submit
    pub fn retain_on_submit(mut self, retain: bool) -> Self {
        self.retain_on_submit = retain;
        self
    }

    /// Set input filter for validation
    pub fn with_filter(mut self, filter_type: InputFilter) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Set maximum length for input
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.max_length = Some(max_length);
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: Some(max_length),
                transform: InputTransform::None,
            });
        }
        self
    }

    /// Set text transformation
    pub fn with_transform(mut self, transform: InputTransform) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.transform = transform;
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: None,
                transform,
            });
        }
        self
    }

    /// Convenience method for numeric-only input (0-9)
    pub fn numeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Numeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for integer input (with optional negative)
    pub fn integer_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Integer,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for decimal input
    pub fn decimal_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Decimal,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphabetic-only input
    pub fn alphabetic_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphabetic,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphanumeric-only input
    pub fn alphanumeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphanumeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    pub fn with_clear_button(mut self) -> Self {
        self.show_clear_button = true;
        self
    }

    pub fn with_marker<M: Component>(self, marker: M) -> TextInputBuilderWithMarker<M> {
        TextInputBuilderWithMarker {
            builder: self,
            marker,
        }
    }

    /// Build and spawn the text input entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_text_input_with_extras(parent, self, |_entity| {})
    }
}

impl<M: Component> TextInputBuilderWithMarker<M> {
    pub fn and_marker<N: Component>(self, marker2: N) -> TextInputBuilderWithTwoMarkers<M, N> {
        TextInputBuilderWithTwoMarkers {
            builder: self.builder,
            marker1: self.marker,
            marker2,
        }
    }

    /// Build and spawn the text input entity with the marker
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_text_input_with_extras(parent, self.builder, |entity| {
            entity.insert(self.marker);
        })
    }
}

impl<M: Component, N: Component> TextInputBuilderWithTwoMarkers<M, N> {
    /// Build and spawn the text input entity with both markers
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_text_input_with_extras(parent, self.builder, |entity| {
            entity.insert(self.marker1);
            entity.insert(self.marker2);
        })
    }
}

/// Convenience function to create a text input builder
pub fn text_input() -> TextInputBuilder {
    TextInputBuilder::new()
}
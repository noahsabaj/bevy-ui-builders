//! CheckboxBuilder for creating interactive checkboxes

use bevy::prelude::*;
use super::types::*;
use crate::styles::{colors, dimensions};

/// Builder for creating styled checkboxes
///
/// # Examples
///
/// ```rust
/// use bevy_ui_builders::prelude::*;
///
/// fn build_checkbox(parent: &mut ChildSpawnerCommands) {
///     CheckboxBuilder::new()
///         .checked(false)
///         .with_label("Remember me")
///         .build(parent);
/// }
/// ```
pub struct CheckboxBuilder {
    checked: bool,
    style: CheckboxStyle,
    label: Option<String>,
    label_on_right: bool,
    size: f32,
}

impl CheckboxBuilder {
    /// Create a new unchecked checkbox
    pub fn new() -> Self {
        Self {
            checked: false,
            style: CheckboxStyle::Primary,
            label: None,
            label_on_right: true,
            size: 20.0,
        }
    }

    /// Set the initial checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the visual style
    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = style;
        self
    }

    /// Add a text label to the checkbox
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether the label appears on the right (default) or left of the checkbox
    pub fn label_on_right(mut self, on_right: bool) -> Self {
        self.label_on_right = on_right;
        self
    }

    /// Set the size of the checkbox box (default: 20px)
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Build the checkbox and spawn it
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let state = if self.checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };

        // Container for checkbox + label
        let container_entity = parent.spawn((
            Node {
                flex_direction: if self.label_on_right {
                    FlexDirection::Row
                } else {
                    FlexDirection::RowReverse
                },
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
        )).id();

        // Spawn the checkbox box itself
        parent.commands().entity(container_entity).with_children(|container| {
            let checkbox_entity = container.spawn((
                Node {
                    width: Val::Px(self.size),
                    height: Val::Px(self.size),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(if state.is_checked() {
                    self.style.checked_color()
                } else {
                    self.style.unchecked_color()
                }),
                BorderColor::all(self.style.border_color()),
                BorderRadius::all(Val::Px(4.0)),
                Checkbox,
                state,
                CheckboxStyleComponent(self.style),
                Interaction::default(),
            )).with_children(|checkbox_box| {
                // Checkmark icon (ASCII X for maximum compatibility)
                checkbox_box.spawn((
                    Text::new("X"),
                    TextFont {
                        font_size: self.size * 0.6,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        display: if state.is_checked() {
                            Display::Flex
                        } else {
                            Display::None
                        },
                        ..default()
                    },
                    CheckboxCheckmark,
                ));
            }).id();

            // Add label if provided
            if let Some(label_text) = self.label {
                container.spawn((
                    Text::new(label_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        // Prevent label from interfering with checkbox clicks
                        ..default()
                    },
                ));
            }
        });

        container_entity
    }
}

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

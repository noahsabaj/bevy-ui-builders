//! DropdownBuilder for creating dropdown select components

use bevy::prelude::*;
use super::types::*;
use crate::styles::{colors, dimensions};
use crate::relationships::BelongsToDropdown;

/// Builder for creating dropdown select components
///
/// # Examples
///
/// ```rust
/// use bevy_ui_builders::prelude::*;
///
/// fn build_dropdown(parent: &mut ChildSpawnerCommands) {
///     DropdownBuilder::new(vec!["Option 1".to_string(), "Option 2".to_string()])
///         .placeholder("Select an option")
///         .build(parent);
/// }
/// ```
pub struct DropdownBuilder {
    options: Vec<String>,
    selected_index: Option<usize>,
    placeholder: String,
    width: Val,
}

impl DropdownBuilder {
    /// Create a new dropdown with the given options
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected_index: None,
            placeholder: "Select an option".to_string(),
            width: Val::Px(200.0),
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the initially selected index
    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    /// Set the width of the dropdown
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the dropdown and spawn it
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let data = DropdownData {
            options: self.options.clone(),
            selected_index: self.selected_index,
            placeholder: self.placeholder.clone(),
        };

        let display_text = data.display_text().to_string();

        // Temporary variable to hold dropdown entity ID
        let mut dropdown_entity = Entity::PLACEHOLDER;

        // Main dropdown container
        let container_id = parent.spawn((
            Node {
                width: self.width,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..default()
            },
            Dropdown,
            DropdownState::Closed,
            data,
        )).with_children(|dropdown| {
            dropdown_entity = dropdown.target_entity();
            // Dropdown button
            dropdown.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(dimensions::INPUT_HEIGHT),
                    padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_TERTIARY),
                BorderColor::all(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_SMALL)),
                DropdownButton,
                Interaction::default(),
            )).with_children(|button| {
                // Selected value text
                button.spawn((
                    Text::new(display_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));

                // Down arrow indicator (ASCII for maximum compatibility)
                button.spawn((
                    Text::new("v"),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
            });

            // Dropdown menu (initially hidden)
            let menu_id = dropdown.spawn((
                Node {
                    width: Val::Percent(100.0),
                    max_height: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(dimensions::INPUT_HEIGHT + 4.0),
                    left: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    display: Display::None, // Hidden by default
                    overflow: Overflow::scroll_y(),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.08, 0.1)), // Fully opaque dark background
                BorderColor::all(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_SMALL)),
                GlobalZIndex(dimensions::Z_INDEX_MODAL), // Use GlobalZIndex to appear above ALL UI elements
                DropdownMenu,
                BelongsToDropdown(dropdown_entity),
            )).with_children(|menu| {
                // Spawn options
                for (index, option) in self.options.iter().enumerate() {
                    menu.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                            ..default()
                        },
                        BackgroundColor(if Some(index) == self.selected_index {
                            Color::srgba(0.3, 0.5, 0.8, 0.3)
                        } else {
                            Color::NONE
                        }),
                        DropdownOption { index },
                        Interaction::default(),
                        BelongsToDropdown(dropdown_entity),
                    )).with_children(|option_container| {
                        option_container.spawn((
                            Text::new(option.clone()),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_NORMAL,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                        ));
                    });
                }
            }).id();
        }).id();

        container_id
    }
}

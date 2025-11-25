//! DropdownBuilder for creating dropdown select components

use bevy::prelude::*;
use super::types::*;
use crate::animation::AnimationCategory;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use crate::relationships::BelongsToDropdown;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

/// Builder for creating dropdown select components
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// fn build_dropdown(parent: &mut ChildSpawnerCommands, theme: Res<UiTheme>) {
///     DropdownBuilder::new(vec!["Option 1".to_string(), "Option 2".to_string()])
///         .themed(&theme)
///         .placeholder("Select an option")
///         .build(parent);
/// }
/// ```
pub struct DropdownBuilder {
    options: Vec<String>,
    selected_index: Option<usize>,
    placeholder: String,
    // Theme-resolved colors (set via .themed())
    themed_colors: Option<DropdownColors>,
    base: BuilderBase,
}

impl DropdownBuilder {
    /// Create a new dropdown with the given options
    pub fn new(options: Vec<String>) -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Px(200.0);
        base.node.flex_direction = FlexDirection::Column;
        base.node.position_type = PositionType::Relative;

        Self {
            options,
            selected_index: None,
            placeholder: "Select an option".to_string(),
            themed_colors: None,
            base,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    ///
    /// # Example
    /// ```ignore
    /// fn setup(mut commands: Commands, theme: Res<UiTheme>) {
    ///     commands.spawn(Node::default()).with_children(|parent| {
    ///         DropdownBuilder::new(vec!["A".into(), "B".into()])
    ///             .themed(&theme)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(DropdownColors::from_theme(theme));
        self
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> DropdownColors {
        self.themed_colors.clone()
            .unwrap_or_else(DropdownColors::default_colors)
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
        self.base.node.width = width;
        self
    }

    /// Build the dropdown and spawn it (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for DropdownBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Resolve colors (themed > default)
        let colors = self.resolve_colors();

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
            self.base.node,
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
                BackgroundColor(colors.button_background),
                BorderColor::all(colors.border),
                BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_SMALL)),
                DropdownButton,
                Interaction::default(),
                Transform::default(),
                AnimationCategory::Button,
            )).with_children(|button| {
                // Selected value text
                button.spawn((
                    Text::new(display_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors.text_primary),
                ));

                // Down arrow indicator (ASCII for maximum compatibility)
                button.spawn((
                    Text::new("v"),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors.text_secondary),
                ));
            });

            // Dropdown menu (initially hidden)
            let _menu_id = dropdown.spawn((
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
                BackgroundColor(colors.menu_background),
                BorderColor::all(colors.border),
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
                            colors.selected_highlight
                        } else {
                            Color::NONE
                        }),
                        DropdownOption { index },
                        Interaction::default(),
                        Transform::default(),
                        AnimationCategory::Button,
                        BelongsToDropdown(dropdown_entity),
                    )).with_children(|option_container| {
                        option_container.spawn((
                            Text::new(option.clone()),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_NORMAL,
                                ..default()
                            },
                            TextColor(colors.text_primary),
                        ));
                    });
                }
            }).id();
        }).id();

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(container_id));
        }

        container_id
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for DropdownBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }
}

//! ButtonBuilder implementation

use bevy::prelude::*;
use crate::animation::{AnimationCategory, DisableAutoAnimation};
use crate::styles::{dimensions, ButtonStyle, ButtonSize};
use crate::theme::UiTheme;
use crate::traits::{InteractiveConfig, UiBuilder, LayoutBuilder, BuilderBase};
use crate::relationships::{InButtonGroup, ButtonGroupMembers};
use super::types::{StyledButton, StateColorSet, SelectableButton, Selected, Active, ButtonSelectionColors, ButtonStateColors};

/// Resolved button colors from theme
#[derive(Clone)]
struct ResolvedButtonColors {
    bg: Color,
    text: Color,
    border: Color,
}

/// Builder for creating buttons with consistent styling
pub struct ButtonBuilder {
    text: String,
    style: ButtonStyle,
    size: ButtonSize,
    icon: Option<String>,
    // Selection state fields
    selectable: bool,
    auto_toggle: bool,
    is_selected: bool,
    is_active: bool,
    button_group: Option<Entity>,
    custom_selection_colors: Option<(StateColorSet, StateColorSet)>, // (selected, active)
    // Theme-resolved colors (set via .themed())
    themed_colors: Option<ResolvedButtonColors>,
    base: BuilderBase,
    /// Interactive/animation configuration
    interactive: InteractiveConfig,
}

impl ButtonBuilder {
    /// Create a new button builder with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::Primary,
            size: ButtonSize::Medium,
            icon: None,
            selectable: false,
            auto_toggle: true,
            is_selected: false,
            is_active: false,
            button_group: None,
            custom_selection_colors: None,
            themed_colors: None,
            base: BuilderBase::new(),
            interactive: InteractiveConfig::new(),
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
    ///         ButtonBuilder::new("Click me")
    ///             .themed(&theme)
    ///             .style(ButtonStyle::Primary)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        let (bg, text, border) = self.style.colors_from_theme(theme);
        self.themed_colors = Some(ResolvedButtonColors { bg, text, border });
        self
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> ResolvedButtonColors {
        self.themed_colors.clone().unwrap_or_else(|| {
            let (bg, text, border) = self.style.colors();
            ResolvedButtonColors { bg, text, border }
        })
    }

    /// Set the button style
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the button size
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set a custom width
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Set a custom height
    pub fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    /// Set the margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    /// Set whether the button is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.interactive.disabled = !enabled;
        self
    }

    /// Add an icon (emoji or symbol)
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Make the button selectable (supports toggle/selection behavior)
    pub fn selectable(mut self) -> Self {
        self.selectable = true;
        self
    }

    /// Set the initial selected state (implies selectable)
    pub fn selected(mut self, selected: bool) -> Self {
        self.selectable = true;
        self.is_selected = selected;
        self
    }

    /// Set the button as active (implies selectable)
    /// Active state is used for current tab/page indicators
    pub fn active(mut self, active: bool) -> Self {
        self.selectable = true;
        self.is_active = active;
        self
    }

    /// Disable auto-toggle behavior (selection must be managed manually)
    pub fn manual_toggle(mut self) -> Self {
        self.auto_toggle = false;
        self
    }

    /// Add this button to a button group for exclusive selection (radio button behavior)
    /// Automatically makes the button selectable
    pub fn in_group(mut self, group_entity: Entity) -> Self {
        self.selectable = true;
        self.button_group = Some(group_entity);
        self
    }

    /// Set custom colors for selected and active states
    /// If not set, colors are auto-generated from the button style
    pub fn selection_colors(mut self, selected: StateColorSet, active: StateColorSet) -> Self {
        self.custom_selection_colors = Some((selected, active));
        self
    }

    /// Build the button entity (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for ButtonBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Resolve colors (themed > default)
        let colors = self.resolve_colors();
        let (padding, font_size, height) = self.size.dimensions();

        let button_width = self.base.node.width; // Default is Auto
        let button_height = if matches!(self.base.node.height, Val::Auto) {
            Val::Px(height)
        } else {
            self.base.node.height
        };

        let mut button = parent.spawn((
            Button,
            Node {
                width: button_width,
                height: button_height,
                margin: self.base.node.margin,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_MEDIUM)),
                padding,
                ..default()
            },
            BackgroundColor(colors.bg),
            BorderColor::all(colors.border),
            BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_MEDIUM)),
            StyledButton,
            Transform::default(), // Required for scale animations
            AnimationCategory::Button, // For auto-animation
        ));

        // Handle animation configuration
        if self.interactive.disable_animation {
            button.insert(DisableAutoAnimation);
        }
        if let Some(custom_anim) = self.interactive.build_animation() {
            button.insert(custom_anim);
        }

        let button_entity = button.id();

        // Handle selectable button setup - prepare the components
        let selectable_setup = if self.selectable {
            // Get base colors for normal state
            let normal_colors = StateColorSet::from_base(colors.bg, colors.border);

            // Generate selection colors (brighter version for selected state)
            let (selected_colors, active_colors) = self.custom_selection_colors.clone()
                .unwrap_or_else(|| {
                    // Selected state: use a brighter/more saturated version
                    let selected_bg = self.style.base_color();
                    let selected_border = self.style.base_color();
                    let selected = StateColorSet::from_base(selected_bg, selected_border);

                    // Active state: even more prominent
                    let active_bg = self.style.hover_color();
                    let active_border = self.style.base_color();
                    let active = StateColorSet::from_base(active_bg, active_border);

                    (selected, active)
                });

            Some((normal_colors, selected_colors, active_colors, self.auto_toggle, self.is_selected, self.is_active, self.button_group))
        } else {
            None
        };

        button.with_children(|button| {
            if let Some(icon) = self.icon {
                // Icon + Text layout
                button.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(dimensions::SPACING_SMALL),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    Pickable::IGNORE, // Don't block button interaction
                )).with_children(|container| {
                    // Icon
                    container.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(colors.text),
                        Pickable::IGNORE, // Don't block button interaction
                    ));

                    // Text
                    container.spawn((
                        Text::new(&self.text),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(colors.text),
                        Pickable::IGNORE, // Don't block button interaction
                    ));
                });
            } else {
                // Just text
                button.spawn((
                    Text::new(&self.text),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(colors.text),
                    Pickable::IGNORE, // Don't block button interaction
                ));
            }
        });
        
        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(button_entity));
        }

        // Apply selectable setup after children are spawned
        if let Some((normal_colors, selected_colors, active_colors, auto_toggle, is_selected, is_active, button_group)) = selectable_setup {
            let mut cmds = parent.commands();
            let mut entity_cmds = cmds.entity(button_entity);

            entity_cmds.insert((
                SelectableButton { auto_toggle },
                ButtonSelectionColors {
                    normal: normal_colors.clone(),
                    selected: selected_colors,
                    active: active_colors,
                },
                ButtonStateColors {
                    normal_bg: normal_colors.normal_bg,
                    hover_bg: normal_colors.hover_bg,
                    pressed_bg: normal_colors.pressed_bg,
                    normal_border: normal_colors.normal_border,
                    hover_border: normal_colors.hover_border,
                    pressed_border: normal_colors.pressed_border,
                },
            ));

            // Handle initial selected state
            if is_selected {
                entity_cmds.insert(Selected);
            }

            // Handle initial active state
            if is_active {
                entity_cmds.insert(Active);
            }

            // Handle button group membership
            if let Some(group_entity) = button_group {
                entity_cmds.insert(InButtonGroup(group_entity));

                // Add this button to the group's members
                let entity_to_add = button_entity;
                cmds.entity(group_entity).entry::<ButtonGroupMembers>()
                    .and_modify(move |mut members| {
                        members.push(entity_to_add);
                    });
            }
        }

        button_entity
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

impl LayoutBuilder for ButtonBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(self, _padding: UiRect) -> Self {
        // Buttons handle their own padding
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

/// Convenience function to create a button builder
pub fn button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text)
}

/// Convenience function for creating a primary button
pub fn primary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Primary)
}

/// Convenience function for creating a secondary button
pub fn secondary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Secondary)
}

/// Convenience function for creating a success button
pub fn success_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Success)
}

/// Convenience function for creating a danger button
pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Danger)
}

/// Convenience function for creating a ghost button
pub fn ghost_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Ghost)
}

// Implement InteractiveBuilder trait for ButtonBuilder
crate::impl_interactive_builder!(ButtonBuilder);

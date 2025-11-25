//! CheckboxBuilder for creating interactive checkboxes

use bevy::prelude::*;
use super::types::*;
use crate::animation::AnimationCategory;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

/// Resolved checkbox colors
#[derive(Clone)]
struct ResolvedCheckboxColors {
    checked: Color,
    unchecked: Color,
    border: Color,
    label: Color,
}

/// Builder for creating styled checkboxes
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// fn build_checkbox(parent: &mut ChildSpawnerCommands, theme: Res<UiTheme>) {
///     CheckboxBuilder::new()
///         .themed(&theme)
///         .checked(false)
///         .with_label("Remember me")
///         .build(parent);
/// }
/// ```
pub struct CheckboxBuilder {
    checked: bool,
    style: CheckboxStyle,
    label: Option<String>,
    size: f32,
    // Theme-resolved values (set via .themed())
    themed_colors: Option<ResolvedCheckboxColors>,
    base: BuilderBase,
}

impl CheckboxBuilder {
    /// Create a new unchecked checkbox
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        // Set default container style
        base.node.flex_direction = FlexDirection::Row;
        base.node.align_items = AlignItems::Center;
        base.node.column_gap = Val::Px(8.0);

        Self {
            checked: false,
            style: CheckboxStyle::Primary,
            label: None,
            size: 20.0,
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
    ///         CheckboxBuilder::new()
    ///             .themed(&theme)
    ///             .with_label("Accept terms")
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(ResolvedCheckboxColors {
            checked: self.style.checked_color_from_theme(theme),
            unchecked: self.style.unchecked_color_from_theme(theme),
            border: self.style.border_color_from_theme(theme),
            label: self.style.label_color_from_theme(theme),
        });
        self
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
        self.base.node.flex_direction = if on_right {
            FlexDirection::Row
        } else {
            FlexDirection::RowReverse
        };
        self
    }

    /// Set the size of the checkbox box (default: 20px)
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Build the checkbox entity (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> ResolvedCheckboxColors {
        self.themed_colors.clone().unwrap_or_else(|| ResolvedCheckboxColors {
            checked: self.style.default_checked_color(),
            unchecked: self.style.default_unchecked_color(),
            border: self.style.default_border_color(),
            label: self.style.default_label_color(),
        })
    }
}

impl UiBuilder for CheckboxBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let state = if self.checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };

        // Resolve colors (themed > default)
        let colors = self.resolve_colors();

        // Container for checkbox + label
        let mut container = parent.spawn(self.base.node);
        let container_entity = container.id();
        let label = self.label.clone();
        let size = self.size;
        let style = self.style;

        // Spawn the checkbox box itself
        container.with_children(|container| {
            let _checkbox_entity = container.spawn((
                Node {
                    width: Val::Px(size),
                    height: Val::Px(size),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(if state.is_checked() {
                    colors.checked
                } else {
                    colors.unchecked
                }),
                BorderColor::all(colors.border),
                BorderRadius::all(Val::Px(4.0)),
                Checkbox,
                state,
                CheckboxStyleComponent(style),
                Interaction::default(),
                Transform::default(),
                AnimationCategory::Button,
            )).with_children(|checkbox_box| {
                // Checkmark icon (ASCII X for maximum compatibility)
                checkbox_box.spawn((
                    Text::new("X"),
                    TextFont {
                        font_size: size * 0.6,
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
            if let Some(label_text) = label {
                container.spawn((
                    Text::new(label_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors.label),
                    Node {
                        // Prevent label from interfering with checkbox clicks
                        ..default()
                    },
                ));
            }
        });

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(container_entity));
        }

        container_entity
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

impl LayoutBuilder for CheckboxBuilder {
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

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

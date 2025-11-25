//! SeparatorBuilder implementation

use bevy::prelude::*;
use crate::dimensions;
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating separators with consistent styling
pub struct SeparatorBuilder {
    orientation: Orientation,
    style: SeparatorStyle,
    color: Option<Color>,
    thickness: Option<f32>,
    length: Val,
    // Theme-resolved values (set via .themed())
    themed_color: Option<Color>,
    base: BuilderBase,
}

impl SeparatorBuilder {
    /// Create a new separator builder
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        base.node.margin = UiRect::vertical(Val::Px(dimensions::SEPARATOR_MARGIN));

        Self {
            orientation: Orientation::Horizontal,
            style: SeparatorStyle::Solid,
            color: None,
            thickness: None,
            length: Val::Percent(100.0),
            themed_color: None,
            base,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_color = Some(self.style.color_from_theme(theme));
        self
    }

    /// Set the orientation (Horizontal/Vertical)
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the visual style (controls thickness)
    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Override the color directly
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Override the thickness directly
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = Some(thickness);
        self
    }

    /// Set the margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    /// Set the length (width for horizontal, height for vertical)
    pub fn length(mut self, length: Val) -> Self {
        self.length = length;
        self
    }

    /// Build the separator (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }

    /// Resolve final color (priority: override > themed > default)
    fn resolve_color(&self) -> Color {
        self.color
            .or(self.themed_color)
            .unwrap_or_else(|| {
                if self.style == SeparatorStyle::Invisible {
                    Color::NONE
                } else {
                    DEFAULT_BORDER_COLOR
                }
            })
    }
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for SeparatorBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        let color = self.resolve_color();
        let thickness = self.thickness.unwrap_or_else(|| self.style.thickness());

        let (default_width, default_height) = match self.orientation {
            Orientation::Horizontal => (self.length, Val::Px(thickness)),
            Orientation::Vertical => (Val::Px(thickness), self.length),
        };

        // Apply defaults if not set in base.node
        if matches!(self.base.node.width, Val::Auto) {
            self.base.node.width = default_width;
        }
        if matches!(self.base.node.height, Val::Auto) {
            self.base.node.height = default_height;
        }

        let entity = parent
            .spawn((
                self.base.node,
                BackgroundColor(color),
                Separator {
                    orientation: self.orientation,
                    style: self.style,
                },
            ))
            .id();

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(entity));
        }

        entity
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

impl LayoutBuilder for SeparatorBuilder {
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

/// Convenience function for creating a horizontal separator
pub fn separator() -> SeparatorBuilder {
    SeparatorBuilder::new()
}

/// Convenience function for creating a vertical separator
pub fn separator_vertical() -> SeparatorBuilder {
    SeparatorBuilder::new().orientation(Orientation::Vertical)
}
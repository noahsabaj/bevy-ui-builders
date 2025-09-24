//! SeparatorBuilder implementation

use bevy::prelude::*;
use crate::dimensions;
use super::types::*;

/// Builder for creating separators with consistent styling
pub struct SeparatorBuilder {
    orientation: Orientation,
    style: SeparatorStyle,
    color: Option<Color>,
    thickness: Option<f32>,
    margin: UiRect,
    length: Val,
}

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            style: SeparatorStyle::Solid,
            color: None,
            thickness: None,
            margin: UiRect::vertical(Val::Px(dimensions::SEPARATOR_MARGIN)),
            length: Val::Percent(100.0),
        }
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Override the color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Override the thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn length(mut self, length: Val) -> Self {
        self.length = length;
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let color = self.color.unwrap_or_else(|| self.style.color());
        let thickness = self.thickness.unwrap_or_else(|| self.style.thickness());

        let (width, height) = match self.orientation {
            Orientation::Horizontal => (self.length, Val::Px(thickness)),
            Orientation::Vertical => (Val::Px(thickness), self.length),
        };

        parent
            .spawn((
                Node {
                    width,
                    height,
                    margin: self.margin,
                    ..default()
                },
                BackgroundColor(color),
                Separator {
                    orientation: self.orientation,
                    style: self.style,
                },
            ))
            .id()
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
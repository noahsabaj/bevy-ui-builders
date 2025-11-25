//! Separator component types and markers

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Orientation for UI elements like separators and layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Horizontal orientation (left to right)
    Horizontal,
    /// Vertical orientation (top to bottom)
    Vertical,
}

/// Component for separators/dividers
#[derive(Component, Debug)]
pub struct Separator {
    /// Orientation of the separator
    pub orientation: Orientation,
    /// Visual style of the separator
    pub style: SeparatorStyle,
}

/// Separator style variants (controls thickness only)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorStyle {
    /// Solid line (default, 1px)
    #[default]
    Solid,
    /// Dashed line (simulated with width)
    Dashed,
    /// Dotted line (simulated with width)
    Dotted,
    /// Thicker line (3px)
    Thick,
    /// Thinner line (0.5px)
    Thin,
    /// Spacing only, no visual
    Invisible,
}

impl SeparatorStyle {
    /// Get the thickness associated with this style
    pub fn thickness(&self) -> f32 {
        match self {
            SeparatorStyle::Thick => 3.0,
            SeparatorStyle::Thin => 0.5,
            SeparatorStyle::Invisible => 0.0,
            _ => 1.0,
        }
    }

    /// Get the color from theme based on style
    pub fn color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            SeparatorStyle::Invisible => Color::NONE,
            _ => theme.colors.border.default,
        }
    }
}

// Default color for when no theme is provided
pub(crate) const DEFAULT_BORDER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
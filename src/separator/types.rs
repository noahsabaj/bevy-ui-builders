//! Separator component types and markers

use bevy::prelude::*;

/// Orientation for UI elements like separators and layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Component for separators/dividers
#[derive(Component, Debug)]
pub struct Separator {
    pub orientation: Orientation,
    pub style: SeparatorStyle,
}

/// Separator style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorStyle {
    #[default]
    Solid,     // Solid line
    Dashed,    // Dashed line (simulated with width)
    Dotted,    // Dotted line (simulated with width)
    Thick,     // Thicker line
    Thin,      // Thinner line
    Invisible, // Spacing only, no visual
}

impl SeparatorStyle {
    pub fn color(&self) -> Color {
        match self {
            SeparatorStyle::Invisible => Color::NONE,
            _ => crate::colors::BORDER_DEFAULT,
        }
    }

    pub fn thickness(&self) -> f32 {
        match self {
            SeparatorStyle::Thick => 3.0,
            SeparatorStyle::Thin => 0.5,
            SeparatorStyle::Invisible => 0.0,
            _ => 1.0,
        }
    }
}
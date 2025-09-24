//! Panel component types and markers

use bevy::prelude::*;

/// Component for panels/containers
#[derive(Component, Debug)]
pub struct Panel {
    pub style: PanelStyle,
}

/// Panel style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelStyle {
    #[default]
    Default,     // Standard panel
    Elevated,    // With shadow/depth
    Transparent, // No background
    Dark,        // Dark background
    Light,       // Light background
    Bordered,    // With visible border
    FullScreen,  // Full screen overlay panel
    Card,        // Card-style panel with subtle background
}

impl PanelStyle {
    pub fn background_color(&self) -> Color {
        match self {
            PanelStyle::Default => crate::colors::BACKGROUND_MEDIUM,
            PanelStyle::Elevated => crate::colors::BACKGROUND_LIGHT,
            PanelStyle::Transparent => Color::NONE,
            PanelStyle::Dark => crate::colors::BACKGROUND_DARK,
            PanelStyle::Light => crate::colors::BACKGROUND_LIGHT,
            PanelStyle::Bordered => crate::colors::BACKGROUND_MEDIUM,
            PanelStyle::FullScreen => Color::srgba(0.0, 0.0, 0.0, 0.8), // Semi-transparent overlay
            PanelStyle::Card => crate::colors::BACKGROUND_LIGHT,
        }
    }

    pub fn border_color(&self) -> Color {
        match self {
            PanelStyle::Bordered => crate::colors::BORDER_DEFAULT,
            PanelStyle::Card => Color::srgba(0.2, 0.2, 0.2, 0.3), // Subtle border for cards
            _ => Color::NONE,
        }
    }

    pub fn border_width(&self) -> Val {
        match self {
            PanelStyle::Bordered => Val::Px(crate::dimensions::BORDER_WIDTH),
            PanelStyle::Card => Val::Px(1.0), // Thin border for cards
            _ => Val::Px(0.0),
        }
    }
}
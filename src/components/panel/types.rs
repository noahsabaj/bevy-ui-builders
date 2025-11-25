//! Panel component types and markers

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Component for panels/containers
#[derive(Component, Debug)]
pub struct Panel {
    /// Visual style of the panel
    pub style: PanelStyle,
}

/// Panel style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelStyle {
    /// Standard panel (default)
    #[default]
    Default,
    /// With shadow/depth
    Elevated,
    /// No background
    Transparent,
    /// Dark background
    Dark,
    /// Light background
    Light,
    /// With visible border
    Bordered,
    /// Full screen overlay panel
    FullScreen,
    /// Card-style panel with subtle background
    Card,
}

impl PanelStyle {
    /// Get the background color from theme
    pub fn background_color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            PanelStyle::Default => theme.colors.surface.primary,
            PanelStyle::Elevated => theme.colors.surface.tertiary,
            PanelStyle::Transparent => Color::NONE,
            PanelStyle::Dark => theme.colors.surface.dark,
            PanelStyle::Light => theme.colors.surface.light,
            PanelStyle::Bordered => theme.colors.surface.primary,
            PanelStyle::FullScreen => theme.colors.overlay,
            PanelStyle::Card => theme.colors.surface.secondary,
        }
    }

    /// Get the border color from theme
    pub fn border_color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            PanelStyle::Bordered => theme.colors.border.default,
            PanelStyle::Card => theme.colors.border.light,
            _ => Color::NONE,
        }
    }

    /// Get the border width associated with this style
    pub fn border_width(&self) -> Val {
        match self {
            PanelStyle::Bordered => Val::Px(crate::dimensions::BORDER_WIDTH),
            PanelStyle::Card => Val::Px(1.0), // Thin border for cards
            _ => Val::Px(0.0),
        }
    }
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND_DEFAULT: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const BACKGROUND_ELEVATED: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const BACKGROUND_DARK: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const BACKGROUND_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BORDER_SUBTLE: Color = Color::srgba(0.2, 0.2, 0.2, 0.3);
}

impl PanelStyle {
    /// Get the default background color (no theme)
    pub(crate) fn default_background_color(&self) -> Color {
        match self {
            PanelStyle::Default => defaults::BACKGROUND_DEFAULT,
            PanelStyle::Elevated => defaults::BACKGROUND_ELEVATED,
            PanelStyle::Transparent => Color::NONE,
            PanelStyle::Dark => defaults::BACKGROUND_DARK,
            PanelStyle::Light => defaults::BACKGROUND_ELEVATED,
            PanelStyle::Bordered => defaults::BACKGROUND_DEFAULT,
            PanelStyle::FullScreen => defaults::BACKGROUND_OVERLAY,
            PanelStyle::Card => defaults::BACKGROUND_ELEVATED,
        }
    }

    /// Get the default border color (no theme)
    pub(crate) fn default_border_color(&self) -> Color {
        match self {
            PanelStyle::Bordered => defaults::BORDER_DEFAULT,
            PanelStyle::Card => defaults::BORDER_SUBTLE,
            _ => Color::NONE,
        }
    }
}
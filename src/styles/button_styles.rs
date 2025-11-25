//! Button style definitions

use bevy::prelude::*;
use super::dimensions;
use crate::theme::UiTheme;

/// Default button colors for dark theme fallback
mod defaults {
    use bevy::prelude::Color;

    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const PRIMARY_HOVER: Color = Color::srgb(0.35, 0.56, 0.96);
    pub const PRIMARY_PRESSED: Color = Color::srgb(0.15, 0.36, 0.76);
    pub const PRIMARY_DARK: Color = Color::srgb(0.15, 0.36, 0.76);

    pub const SECONDARY: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const SECONDARY_HOVER: Color = Color::srgb(0.35, 0.35, 0.35);
    pub const SECONDARY_PRESSED: Color = Color::srgb(0.15, 0.15, 0.15);

    pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
    pub const DANGER_HOVER: Color = Color::srgb(0.96, 0.35, 0.35);
    pub const DANGER_PRESSED: Color = Color::srgb(0.76, 0.15, 0.15);
    pub const DANGER_DARK: Color = Color::srgb(0.76, 0.15, 0.15);

    pub const SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);
    pub const SUCCESS_HOVER: Color = Color::srgb(0.35, 0.86, 0.35);
    pub const SUCCESS_PRESSED: Color = Color::srgb(0.15, 0.66, 0.15);
    pub const SUCCESS_DARK: Color = Color::srgb(0.15, 0.66, 0.15);

    pub const WARNING: Color = Color::srgb(0.96, 0.76, 0.05);
    pub const WARNING_HOVER: Color = Color::srgb(1.0, 0.86, 0.15);
    pub const WARNING_PRESSED: Color = Color::srgb(0.86, 0.66, 0.0);

    pub const GHOST_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.20);
    pub const GHOST_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.35);

    pub const TEXT_ON_PRIMARY: Color = Color::WHITE;
    pub const TEXT_ON_SECONDARY: Color = Color::WHITE;
    pub const TEXT_ON_SUCCESS: Color = Color::WHITE;
    pub const TEXT_ON_DANGER: Color = Color::WHITE;
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);

    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
}

/// Button style variants for consistent theming
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Primary action buttons (blue)
    Primary,
    /// Secondary action buttons (gray)
    Secondary,
    /// Destructive action buttons (red)
    Danger,
    /// Positive action buttons (green)
    Success,
    /// Cautionary action buttons (yellow)
    Warning,
    /// Transparent with border only
    Ghost,
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small buttons for compact UI
    Small,
    /// Default button size
    Medium,
    /// Large buttons for prominent actions
    Large,
    /// Extra large buttons for hero sections
    XLarge,
}

impl ButtonStyle {
    /// Get the base color for this button style
    pub fn base_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => defaults::PRIMARY,
            ButtonStyle::Secondary => defaults::SECONDARY,
            ButtonStyle::Danger => defaults::DANGER,
            ButtonStyle::Success => defaults::SUCCESS,
            ButtonStyle::Warning => defaults::WARNING,
            ButtonStyle::Ghost => Color::NONE,
        }
    }

    /// Get the hover color for this button style
    pub fn hover_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => defaults::PRIMARY_HOVER,
            ButtonStyle::Secondary => defaults::SECONDARY_HOVER,
            ButtonStyle::Danger => defaults::DANGER_HOVER,
            ButtonStyle::Success => defaults::SUCCESS_HOVER,
            ButtonStyle::Warning => defaults::WARNING_HOVER,
            ButtonStyle::Ghost => defaults::GHOST_HOVER,
        }
    }

    /// Get the pressed color for this button style
    pub fn pressed_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => defaults::PRIMARY_PRESSED,
            ButtonStyle::Secondary => defaults::SECONDARY_PRESSED,
            ButtonStyle::Danger => defaults::DANGER_PRESSED,
            ButtonStyle::Success => defaults::SUCCESS_PRESSED,
            ButtonStyle::Warning => defaults::WARNING_PRESSED,
            ButtonStyle::Ghost => defaults::GHOST_PRESSED,
        }
    }

    /// Get the text color for this button style
    pub fn text_color(&self) -> Color {
        match self {
            ButtonStyle::Ghost => defaults::TEXT_SECONDARY,
            _ => Color::WHITE,
        }
    }

    /// Get the border color for this button style
    pub fn border_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => defaults::PRIMARY_DARK,
            ButtonStyle::Secondary => defaults::BORDER_DEFAULT,
            ButtonStyle::Danger => defaults::DANGER_DARK,
            ButtonStyle::Success => defaults::SUCCESS_DARK,
            ButtonStyle::Warning => defaults::WARNING_PRESSED,
            ButtonStyle::Ghost => defaults::BORDER_DEFAULT,
        }
    }
    /// Get the colors for this button style (bg, text, border)
    pub fn colors(&self) -> (Color, Color, Color) {
        match self {
            ButtonStyle::Primary => (defaults::PRIMARY, defaults::TEXT_ON_PRIMARY, defaults::PRIMARY_DARK),
            ButtonStyle::Secondary => (defaults::SECONDARY, defaults::TEXT_ON_SECONDARY, defaults::BORDER_DEFAULT),
            ButtonStyle::Success => (defaults::SUCCESS, defaults::TEXT_ON_SUCCESS, defaults::SUCCESS_DARK),
            ButtonStyle::Danger => (defaults::DANGER, defaults::TEXT_ON_DANGER, defaults::DANGER_DARK),
            ButtonStyle::Warning => (defaults::WARNING, Color::BLACK, defaults::WARNING_PRESSED),
            ButtonStyle::Ghost => (Color::NONE, defaults::TEXT_SECONDARY, defaults::BORDER_DEFAULT),
        }
    }

    // ============================================================================
    // Theme-aware methods
    // ============================================================================

    /// Get the colors for this button style from theme (bg, text, border)
    pub fn colors_from_theme(&self, theme: &UiTheme) -> (Color, Color, Color) {
        match self {
            ButtonStyle::Primary => (
                theme.colors.primary.base,
                theme.colors.primary.on_color,
                theme.colors.primary.pressed,
            ),
            ButtonStyle::Secondary => (
                theme.colors.secondary.base,
                theme.colors.secondary.on_color,
                theme.colors.border.default,
            ),
            ButtonStyle::Success => (
                theme.colors.success.base,
                theme.colors.success.on_color,
                theme.colors.success.pressed,
            ),
            ButtonStyle::Danger => (
                theme.colors.danger.base,
                theme.colors.danger.on_color,
                theme.colors.danger.pressed,
            ),
            ButtonStyle::Warning => (
                theme.colors.warning.base,
                Color::BLACK,
                theme.colors.warning.pressed,
            ),
            ButtonStyle::Ghost => (
                Color::NONE,
                theme.colors.text.secondary,
                theme.colors.border.default,
            ),
        }
    }

    /// Get the hover color for this button style from theme
    pub fn hover_color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            ButtonStyle::Primary => theme.colors.primary.hover,
            ButtonStyle::Secondary => theme.colors.secondary.hover,
            ButtonStyle::Danger => theme.colors.danger.hover,
            ButtonStyle::Success => theme.colors.success.hover,
            ButtonStyle::Warning => theme.colors.warning.hover,
            ButtonStyle::Ghost => theme.colors.ghost.hover,
        }
    }

    /// Get the pressed color for this button style from theme
    pub fn pressed_color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            ButtonStyle::Primary => theme.colors.primary.pressed,
            ButtonStyle::Secondary => theme.colors.secondary.pressed,
            ButtonStyle::Danger => theme.colors.danger.pressed,
            ButtonStyle::Success => theme.colors.success.pressed,
            ButtonStyle::Warning => theme.colors.warning.pressed,
            ButtonStyle::Ghost => theme.colors.ghost.pressed,
        }
    }
}

impl ButtonSize {
    /// Get the dimensions (padding, font_size, height) for this button size
    pub fn dimensions(&self) -> (UiRect, f32, f32) {
        let (_width, height) = match self {
            ButtonSize::Small => (dimensions::BUTTON_WIDTH_SMALL, dimensions::BUTTON_HEIGHT_SMALL),
            ButtonSize::Medium => (dimensions::BUTTON_WIDTH_MEDIUM, dimensions::BUTTON_HEIGHT_MEDIUM),
            ButtonSize::Large => (dimensions::BUTTON_WIDTH_LARGE, dimensions::BUTTON_HEIGHT_LARGE),
            ButtonSize::XLarge => (dimensions::BUTTON_WIDTH_XLARGE, dimensions::BUTTON_HEIGHT_XLARGE),
        };
        
        let font_size = match self {
            ButtonSize::Small => dimensions::FONT_SIZE_SMALL,
            ButtonSize::Medium => dimensions::FONT_SIZE_MEDIUM,
            ButtonSize::Large => dimensions::FONT_SIZE_LARGE,
            ButtonSize::XLarge => dimensions::FONT_SIZE_XLARGE,
        };

        let padding = match self {
            ButtonSize::Small => UiRect::horizontal(Val::Px(dimensions::PADDING_SMALL)),
            ButtonSize::Medium => UiRect::horizontal(Val::Px(dimensions::PADDING_MEDIUM)),
            ButtonSize::Large => UiRect::horizontal(Val::Px(dimensions::PADDING_LARGE)),
            ButtonSize::XLarge => UiRect::horizontal(Val::Px(dimensions::PADDING_LARGE * 1.5)),
        };

        (padding, font_size, height)
    }
}
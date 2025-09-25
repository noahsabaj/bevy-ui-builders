//! Button style definitions

use bevy::prelude::*;
use super::colors;
use super::dimensions;

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
            ButtonStyle::Primary => colors::PRIMARY,
            ButtonStyle::Secondary => colors::SECONDARY,
            ButtonStyle::Danger => colors::DANGER,
            ButtonStyle::Success => colors::SUCCESS,
            ButtonStyle::Warning => colors::WARNING,
            ButtonStyle::Ghost => Color::NONE,
        }
    }

    /// Get the hover color for this button style
    pub fn hover_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY_HOVER,
            ButtonStyle::Secondary => colors::SECONDARY_HOVER,
            ButtonStyle::Danger => colors::DANGER_HOVER,
            ButtonStyle::Success => colors::SUCCESS_HOVER,
            ButtonStyle::Warning => colors::WARNING_HOVER,
            ButtonStyle::Ghost => colors::GHOST_HOVER,
        }
    }

    /// Get the pressed color for this button style
    pub fn pressed_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY_PRESSED,
            ButtonStyle::Secondary => colors::SECONDARY_PRESSED,
            ButtonStyle::Danger => colors::DANGER_PRESSED,
            ButtonStyle::Success => colors::SUCCESS_PRESSED,
            ButtonStyle::Warning => colors::WARNING_PRESSED,
            ButtonStyle::Ghost => colors::GHOST_PRESSED,
        }
    }

    /// Get the text color for this button style
    pub fn text_color(&self) -> Color {
        match self {
            ButtonStyle::Ghost => colors::TEXT_SECONDARY,
            _ => Color::WHITE,
        }
    }

    /// Get the border color for this button style
    pub fn border_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY_DARK,
            ButtonStyle::Secondary => colors::BORDER_DEFAULT,
            ButtonStyle::Danger => colors::DANGER_DARK,
            ButtonStyle::Success => colors::SUCCESS_DARK,
            ButtonStyle::Warning => colors::WARNING_PRESSED,
            ButtonStyle::Ghost => colors::BORDER_DEFAULT,
        }
    }
}

impl ButtonSize {
    /// Get the width for this button size
    pub fn width(&self) -> Val {
        match self {
            ButtonSize::Small => Val::Px(dimensions::BUTTON_WIDTH_SMALL),
            ButtonSize::Medium => Val::Px(dimensions::BUTTON_WIDTH_MEDIUM),
            ButtonSize::Large => Val::Px(dimensions::BUTTON_WIDTH_LARGE),
            ButtonSize::XLarge => Val::Px(dimensions::BUTTON_WIDTH_XLARGE),
        }
    }

    /// Get the height for this button size
    pub fn height(&self) -> Val {
        match self {
            ButtonSize::Small => Val::Px(dimensions::BUTTON_HEIGHT_SMALL),
            ButtonSize::Medium => Val::Px(dimensions::BUTTON_HEIGHT_MEDIUM),
            ButtonSize::Large => Val::Px(dimensions::BUTTON_HEIGHT_LARGE),
            ButtonSize::XLarge => Val::Px(dimensions::BUTTON_HEIGHT_XLARGE),
        }
    }

    /// Get the font size for this button size
    pub fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Small => dimensions::FONT_SIZE_SMALL,
            ButtonSize::Medium => dimensions::FONT_SIZE_MEDIUM,
            ButtonSize::Large => dimensions::FONT_SIZE_LARGE,
            ButtonSize::XLarge => dimensions::FONT_SIZE_XLARGE,
        }
    }
}
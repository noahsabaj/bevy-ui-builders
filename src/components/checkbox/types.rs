//! Checkbox component types and styles

use bevy::prelude::*;

use crate::theme::{UiTheme, SemanticVariant};

/// Marker component for checkbox entities
#[derive(Component, Debug, Clone, Copy)]
pub struct Checkbox;

/// State of a checkbox (checked or unchecked)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    /// Checkbox is unchecked
    Unchecked,
    /// Checkbox is checked
    Checked,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self::Unchecked
    }
}

impl CheckboxState {
    /// Toggle the checkbox state
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Unchecked => Self::Checked,
            Self::Checked => Self::Unchecked,
        };
    }

    /// Check if the checkbox is checked
    pub fn is_checked(&self) -> bool {
        matches!(self, Self::Checked)
    }
}

/// Visual style variants for checkboxes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxStyle {
    /// Default gray style
    Default,
    /// Primary blue style (recommended for most use cases)
    Primary,
    /// Success green style
    Success,
    /// Danger red style
    Danger,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self::Primary
    }
}

impl CheckboxStyle {
    /// Convert to SemanticVariant
    pub fn to_variant(&self) -> SemanticVariant {
        match self {
            Self::Default => SemanticVariant::Secondary,
            Self::Primary => SemanticVariant::Primary,
            Self::Success => SemanticVariant::Success,
            Self::Danger => SemanticVariant::Danger,
        }
    }

    /// Get the background color for the checked state from theme
    pub fn checked_color_from_theme(&self, theme: &UiTheme) -> Color {
        match self {
            Self::Default => theme.colors.secondary.base,
            Self::Primary => theme.colors.primary.base,
            Self::Success => theme.colors.success.base,
            Self::Danger => theme.colors.danger.base,
        }
    }

    /// Get the background color for the unchecked state from theme
    pub fn unchecked_color_from_theme(&self, theme: &UiTheme) -> Color {
        theme.colors.surface.secondary
    }

    /// Get the border color from theme
    pub fn border_color_from_theme(&self, theme: &UiTheme) -> Color {
        theme.colors.border.default
    }

    /// Get the label text color from theme
    pub fn label_color_from_theme(&self, theme: &UiTheme) -> Color {
        theme.colors.text.primary
    }
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const SECONDARY: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);
    pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
    pub const BACKGROUND: Color = Color::srgb(0.08, 0.08, 0.1);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
}

impl CheckboxStyle {
    /// Get the default checked color (no theme)
    /// Also used by systems at runtime
    pub fn default_checked_color(&self) -> Color {
        match self {
            Self::Default => defaults::SECONDARY,
            Self::Primary => defaults::PRIMARY,
            Self::Success => defaults::SUCCESS,
            Self::Danger => defaults::DANGER,
        }
    }

    /// Get the default unchecked color (no theme)
    /// Also used by systems at runtime
    pub fn default_unchecked_color(&self) -> Color {
        defaults::BACKGROUND
    }

    /// Get the default border color (no theme)
    pub(crate) fn default_border_color(&self) -> Color {
        defaults::BORDER
    }

    /// Get the default label text color (no theme)
    pub(crate) fn default_label_color(&self) -> Color {
        defaults::TEXT_PRIMARY
    }

    /// Get the checked color (uses default, for backwards compatibility with systems)
    pub fn checked_color(&self) -> Color {
        self.default_checked_color()
    }

    /// Get the unchecked color (uses default, for backwards compatibility with systems)
    pub fn unchecked_color(&self) -> Color {
        self.default_unchecked_color()
    }

    /// Get the border color (uses default, for backwards compatibility with systems)
    pub fn border_color(&self) -> Color {
        self.default_border_color()
    }
}

/// Component that stores the visual style of a checkbox
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxStyleComponent(pub CheckboxStyle);

/// Marker component for the checkmark icon inside a checkbox
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxCheckmark;

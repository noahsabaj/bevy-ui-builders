//! Dropdown component types and state

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Marker component for dropdown entities
#[derive(Component, Debug, Clone, Copy)]
pub struct Dropdown;

/// Marker component for the dropdown button (clickable trigger)
#[derive(Component, Debug, Clone, Copy)]
pub struct DropdownButton;

/// Marker component for the dropdown menu container
#[derive(Component, Debug, Clone, Copy)]
pub struct DropdownMenu;

/// Marker component for individual dropdown options
#[derive(Component, Debug, Clone, Copy)]
pub struct DropdownOption {
    /// Index of this option in the dropdown
    pub index: usize,
}

/// State of the dropdown (open or closed)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownState {
    /// Dropdown menu is closed
    Closed,
    /// Dropdown menu is open
    Open,
}

impl Default for DropdownState {
    fn default() -> Self {
        Self::Closed
    }
}

/// Component storing dropdown configuration and current selection
#[derive(Component, Debug, Clone)]
pub struct DropdownData {
    /// Available options
    pub options: Vec<String>,
    /// Currently selected option index
    pub selected_index: Option<usize>,
    /// Placeholder text when nothing is selected
    pub placeholder: String,
}

impl DropdownData {
    /// Get the currently selected value
    pub fn selected_value(&self) -> Option<&str> {
        self.selected_index
            .and_then(|idx| self.options.get(idx))
            .map(|s| s.as_str())
    }

    /// Get the text to display (selected value or placeholder)
    pub fn display_text(&self) -> &str {
        self.selected_value().unwrap_or(&self.placeholder)
    }
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.08, 0.08, 0.1);
    pub const BUTTON_BACKGROUND: Color = Color::srgb(0.15, 0.15, 0.18);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const SELECTED_HIGHLIGHT: Color = Color::srgba(0.3, 0.5, 0.8, 0.3);
}

/// Resolved dropdown colors from theme
#[derive(Clone)]
pub struct DropdownColors {
    /// Button background color
    pub button_background: Color,
    /// Menu background color
    pub menu_background: Color,
    /// Border color
    pub border: Color,
    /// Primary text color
    pub text_primary: Color,
    /// Secondary text color (arrow indicator)
    pub text_secondary: Color,
    /// Selected option highlight
    pub selected_highlight: Color,
}

impl DropdownColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        use bevy::color::Alpha;
        Self {
            button_background: theme.colors.surface.tertiary,
            menu_background: theme.colors.surface.secondary,
            border: theme.colors.border.default,
            text_primary: theme.colors.text.primary,
            text_secondary: theme.colors.text.secondary,
            selected_highlight: theme.colors.primary.base.with_alpha(0.3),
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            button_background: defaults::BUTTON_BACKGROUND,
            menu_background: defaults::BACKGROUND,
            border: defaults::BORDER,
            text_primary: defaults::TEXT_PRIMARY,
            text_secondary: defaults::TEXT_SECONDARY,
            selected_highlight: defaults::SELECTED_HIGHLIGHT,
        }
    }
}

//! Checkbox component types and styles

use bevy::prelude::*;

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
    /// Get the background color for the checked state
    pub fn checked_color(&self) -> Color {
        match self {
            Self::Default => crate::styles::colors::SECONDARY,
            Self::Primary => crate::styles::colors::PRIMARY,
            Self::Success => crate::styles::colors::SUCCESS,
            Self::Danger => crate::styles::colors::DANGER,
        }
    }

    /// Get the background color for the unchecked state
    pub fn unchecked_color(&self) -> Color {
        crate::styles::colors::BACKGROUND_SECONDARY
    }

    /// Get the border color
    pub fn border_color(&self) -> Color {
        crate::styles::colors::BORDER_DEFAULT
    }
}

/// Component that stores the visual style of a checkbox
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxStyleComponent(pub CheckboxStyle);

/// Marker component for the checkmark icon inside a checkbox
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxCheckmark;

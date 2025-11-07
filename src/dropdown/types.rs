//! Dropdown component types and state

use bevy::prelude::*;

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

//! Tab view types and components

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Component marking a tab view container
#[derive(Component, Clone, Debug)]
pub struct TabView {
    /// Currently active tab index
    pub active_tab: usize,
    /// Total number of tabs
    pub tab_count: usize,
}

/// Component marking an individual tab button
#[derive(Component, Clone, Debug)]
pub struct TabButton {
    /// The tab view this button belongs to
    pub tab_view: Entity,
    /// The index of this tab
    pub index: usize,
}

/// Component marking a tab content panel
#[derive(Component, Clone, Debug)]
pub struct TabContent {
    /// The tab view this content belongs to
    pub tab_view: Entity,
    /// The index of this content panel
    pub index: usize,
}

/// Position of the tab buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabPosition {
    /// Tabs at the top (default)
    #[default]
    Top,
    /// Tabs at the bottom
    Bottom,
    /// Tabs on the left
    Left,
    /// Tabs on the right
    Right,
}

/// Style variants for tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabStyle {
    /// Default line-based tabs
    #[default]
    Line,
    /// Boxed/contained tabs
    Boxed,
    /// Pill-shaped tabs
    Pills,
    /// Segmented control style
    Segmented,
}

/// Message emitted when a tab is selected
#[derive(Message, Clone, Debug)]
pub struct TabSelectedEvent {
    /// The tab view entity
    pub tab_view: Entity,
    /// The selected tab index
    pub index: usize,
    /// The previous tab index
    pub previous_index: usize,
}

/// Configuration for a single tab
#[derive(Clone, Debug)]
pub struct TabConfig {
    /// Tab label text
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether this tab is disabled
    pub disabled: bool,
    /// Optional badge count
    pub badge: Option<u32>,
}

impl TabConfig {
    /// Create a new tab configuration
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
            badge: None,
        }
    }

    /// Add an icon to the tab
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark the tab as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Add a badge count
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.14);
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_ON_PRIMARY: Color = Color::WHITE;
    pub const TEXT_ON_DANGER: Color = Color::WHITE;
    pub const TEXT_DISABLED: Color = Color::srgb(0.4, 0.4, 0.4);
}

/// Resolved tab colors from theme
#[derive(Clone)]
pub struct TabColors {
    /// Tab container background
    pub background: Color,
    /// Active tab background
    pub active_background: Color,
    /// Active tab text color
    pub active_text: Color,
    /// Inactive tab text color
    pub inactive_text: Color,
    /// Disabled tab text color
    pub disabled_text: Color,
    /// Badge background
    pub badge_background: Color,
    /// Badge text color
    pub badge_text: Color,
}

impl TabColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        Self {
            background: theme.colors.surface.secondary,
            active_background: theme.colors.primary.base,
            active_text: theme.colors.primary.on_color,
            inactive_text: theme.colors.text.primary,
            disabled_text: theme.colors.text.disabled,
            badge_background: theme.colors.danger.base,
            badge_text: theme.colors.danger.on_color,
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            background: defaults::BACKGROUND,
            active_background: defaults::PRIMARY,
            active_text: defaults::TEXT_ON_PRIMARY,
            inactive_text: defaults::TEXT_PRIMARY,
            disabled_text: defaults::TEXT_DISABLED,
            badge_background: defaults::DANGER,
            badge_text: defaults::TEXT_ON_DANGER,
        }
    }
}

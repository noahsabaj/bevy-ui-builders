//! Context menu types and components

use bevy::prelude::*;

use crate::theme::UiTheme;

/// A context menu item
#[derive(Clone, Debug)]
pub enum MenuItem {
    /// An action item
    Action {
        /// The label text
        label: String,
        /// Optional keyboard shortcut display
        shortcut: Option<String>,
        /// Whether the item is disabled
        disabled: bool,
        /// Unique identifier for the action
        id: String,
    },
    /// A visual separator
    Separator,
    /// A submenu
    Submenu {
        /// The submenu label
        label: String,
        /// The submenu items
        items: Vec<MenuItem>,
    },
    /// A checkbox item
    Checkbox {
        /// The label text
        label: String,
        /// Whether it's checked
        checked: bool,
        /// Unique identifier
        id: String,
    },
}

impl MenuItem {
    /// Create a new action item
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Action {
            label: label.into(),
            shortcut: None,
            disabled: false,
            id: id.into(),
        }
    }

    /// Create a new action with shortcut
    pub fn action_with_shortcut(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: impl Into<String>,
    ) -> Self {
        Self::Action {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            disabled: false,
            id: id.into(),
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Create a submenu
    pub fn submenu(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self::Submenu {
            label: label.into(),
            items,
        }
    }

    /// Create a checkbox item
    pub fn checkbox(id: impl Into<String>, label: impl Into<String>, checked: bool) -> Self {
        Self::Checkbox {
            label: label.into(),
            checked,
            id: id.into(),
        }
    }

    /// Add disabled state to an action
    pub fn disabled(self) -> Self {
        match self {
            Self::Action { label, shortcut, id, .. } => Self::Action {
                label,
                shortcut,
                disabled: true,
                id,
            },
            other => other,
        }
    }
}

/// Component marking a context menu trigger
#[derive(Component, Clone, Debug)]
pub struct ContextMenuTrigger {
    /// The menu items to show
    pub items: Vec<MenuItem>,
}

/// Component marking a visible context menu
#[derive(Component, Clone, Debug)]
pub struct ContextMenu {
    /// The trigger entity this menu belongs to
    pub trigger: Entity,
    /// Position where the menu was opened
    pub position: Vec2,
}

/// Component marking a menu item entity
#[derive(Component, Clone, Debug)]
pub struct ContextMenuItem {
    /// The menu this item belongs to
    pub menu: Entity,
    /// The item definition
    pub item: MenuItem,
    /// Index in the menu
    pub index: usize,
}

/// Component marking a submenu container
#[derive(Component, Clone, Debug)]
pub struct SubmenuContainer {
    /// Parent menu entity
    pub parent_menu: Entity,
    /// Index of the submenu trigger in parent
    pub trigger_index: usize,
}

/// Message emitted when a context menu action is selected
#[derive(Message, Clone, Debug)]
pub struct ContextMenuActionEvent {
    /// The action identifier
    pub id: String,
    /// The trigger entity that opened the menu
    pub trigger: Entity,
}

/// Message emitted when a checkbox state changes
#[derive(Message, Clone, Debug)]
pub struct ContextMenuCheckboxEvent {
    /// The checkbox identifier
    pub id: String,
    /// The new checked state
    pub checked: bool,
    /// The trigger entity
    pub trigger: Entity,
}

/// Global context menu settings
#[derive(Resource, Clone, Debug)]
pub struct ContextMenuSettings {
    /// Z-index for context menus
    pub z_index: i32,
    /// Submenu offset (overlap with parent)
    pub submenu_offset: f32,
    /// Animation duration
    pub animation_duration: f32,
}

impl Default for ContextMenuSettings {
    fn default() -> Self {
        Self {
            z_index: 1500,
            submenu_offset: -4.0,
            animation_duration: 0.1,
        }
    }
}

/// Resource tracking the currently open context menu
#[derive(Resource, Default)]
pub struct OpenContextMenu {
    /// The currently open menu entity (if any)
    pub menu: Option<Entity>,
    /// The trigger entity
    pub trigger: Option<Entity>,
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.14);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const TEXT_DISABLED: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const GHOST_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.05);
    pub const GHOST_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.1);
}

/// Resolved context menu colors from theme
#[derive(Clone)]
pub struct ContextMenuColors {
    /// Menu background color
    pub background: Color,
    /// Border color
    pub border: Color,
    /// Primary text color
    pub text_primary: Color,
    /// Secondary text color
    pub text_secondary: Color,
    /// Muted text color (shortcuts)
    pub text_muted: Color,
    /// Disabled text color
    pub text_disabled: Color,
    /// Primary/accent color (checkmarks)
    pub primary: Color,
    /// Hover background color
    pub hover: Color,
    /// Pressed background color
    pub pressed: Color,
}

impl ContextMenuColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        Self {
            background: theme.colors.surface.secondary,
            border: theme.colors.border.default,
            text_primary: theme.colors.text.primary,
            text_secondary: theme.colors.text.secondary,
            text_muted: theme.colors.text.muted,
            text_disabled: theme.colors.text.disabled,
            primary: theme.colors.primary.base,
            hover: theme.colors.ghost.hover,
            pressed: theme.colors.ghost.pressed,
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            background: defaults::BACKGROUND,
            border: defaults::BORDER,
            text_primary: defaults::TEXT_PRIMARY,
            text_secondary: defaults::TEXT_SECONDARY,
            text_muted: defaults::TEXT_MUTED,
            text_disabled: defaults::TEXT_DISABLED,
            primary: defaults::PRIMARY,
            hover: defaults::GHOST_HOVER,
            pressed: defaults::GHOST_PRESSED,
        }
    }
}

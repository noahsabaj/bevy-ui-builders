//! ContextMenuBuilder implementation

use bevy::prelude::*;
use crate::traits::BuilderBase;
use super::types::*;

/// Builder for creating context menus
///
/// Context menus are attached to UI elements and shown on right-click.
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// // Attach a context menu to an element
/// ContextMenuBuilder::new()
///     .action("copy", "Copy", Some("Ctrl+C"))
///     .action("paste", "Paste", Some("Ctrl+V"))
///     .separator()
///     .submenu("Export", |sub| {
///         sub.action("export_png", "PNG", None)
///            .action("export_jpg", "JPEG", None);
///     })
///     .build_for(&mut commands, target_entity);
/// ```
pub struct ContextMenuBuilder {
    items: Vec<MenuItem>,
    base: BuilderBase,
}

impl ContextMenuBuilder {
    /// Create a new context menu builder
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            base: BuilderBase::new(),
        }
    }

    /// Add an action item
    pub fn action(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self {
        let item = if let Some(sc) = shortcut {
            MenuItem::action_with_shortcut(id, label, sc)
        } else {
            MenuItem::action(id, label)
        };
        self.items.push(item);
        self
    }

    /// Add a disabled action item
    pub fn action_disabled(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self {
        let item = if let Some(sc) = shortcut {
            MenuItem::action_with_shortcut(id, label, sc).disabled()
        } else {
            MenuItem::action(id, label).disabled()
        };
        self.items.push(item);
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }

    /// Add a submenu
    pub fn submenu(
        mut self,
        label: impl Into<String>,
        builder: impl FnOnce(SubmenuBuilder) -> SubmenuBuilder,
    ) -> Self {
        let submenu_builder = SubmenuBuilder::new();
        let submenu = builder(submenu_builder);
        self.items.push(MenuItem::submenu(label, submenu.items));
        self
    }

    /// Add a checkbox item
    pub fn checkbox(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        checked: bool,
    ) -> Self {
        self.items.push(MenuItem::checkbox(id, label, checked));
        self
    }

    /// Add a pre-built menu item
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Build and attach the context menu to an existing entity
    pub fn build_for(self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(ContextMenuTrigger {
            items: self.items,
        });
    }

    /// Get the menu items (for manual handling)
    pub fn into_items(self) -> Vec<MenuItem> {
        self.items
    }
}

impl Default for ContextMenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for submenu items
pub struct SubmenuBuilder {
    items: Vec<MenuItem>,
}

impl SubmenuBuilder {
    /// Create a new submenu builder
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an action item
    pub fn action(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self {
        let item = if let Some(sc) = shortcut {
            MenuItem::action_with_shortcut(id, label, sc)
        } else {
            MenuItem::action(id, label)
        };
        self.items.push(item);
        self
    }

    /// Add a disabled action item
    pub fn action_disabled(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self {
        let item = if let Some(sc) = shortcut {
            MenuItem::action_with_shortcut(id, label, sc).disabled()
        } else {
            MenuItem::action(id, label).disabled()
        };
        self.items.push(item);
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }

    /// Add a nested submenu
    pub fn submenu(
        mut self,
        label: impl Into<String>,
        builder: impl FnOnce(SubmenuBuilder) -> SubmenuBuilder,
    ) -> Self {
        let submenu_builder = SubmenuBuilder::new();
        let submenu = builder(submenu_builder);
        self.items.push(MenuItem::submenu(label, submenu.items));
        self
    }

    /// Add a checkbox item
    pub fn checkbox(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        checked: bool,
    ) -> Self {
        self.items.push(MenuItem::checkbox(id, label, checked));
        self
    }
}

impl Default for SubmenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create a context menu builder
pub fn context_menu() -> ContextMenuBuilder {
    ContextMenuBuilder::new()
}

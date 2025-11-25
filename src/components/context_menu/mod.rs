//! Context menu component
//!
//! Provides right-click context menus for UI elements.
//!
//! # Features
//!
//! - Action items with optional keyboard shortcuts
//! - Checkbox items
//! - Separators
//! - Nested submenus
//! - Event-based action handling
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::prelude::*;
//!
//! // Attach a context menu to an element
//! fn setup_context_menu(mut commands: Commands) {
//!     let panel = PanelBuilder::new().build(&mut commands);
//!
//!     ContextMenuBuilder::new()
//!         .action("copy", "Copy", Some("Ctrl+C"))
//!         .action("paste", "Paste", Some("Ctrl+V"))
//!         .separator()
//!         .checkbox("show_grid", "Show Grid", true)
//!         .submenu("Export", |sub| {
//!             sub.action("export_png", "PNG", None)
//!                .action("export_jpg", "JPEG", None)
//!         })
//!         .build_for(&mut commands, panel);
//! }
//!
//! // Handle context menu events
//! fn handle_actions(mut events: EventReader<ContextMenuActionEvent>) {
//!     for event in events.read() {
//!         match event.id.as_str() {
//!             "copy" => { /* handle copy */ }
//!             "paste" => { /* handle paste */ }
//!             _ => {}
//!         }
//!     }
//! }
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{ContextMenuBuilder, SubmenuBuilder, context_menu};
pub use plugin::ContextMenuPlugin;
pub use types::{
    MenuItem, ContextMenuTrigger, ContextMenu, ContextMenuItem,
    SubmenuContainer, ContextMenuActionEvent, ContextMenuCheckboxEvent,
    ContextMenuSettings, OpenContextMenu,
};

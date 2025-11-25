//! Inventory grid component
//!
//! Provides a grid-based inventory system with drag-and-drop support.
//!
//! # Features
//!
//! - Configurable grid dimensions
//! - Customizable slot size and spacing
//! - Drag and drop support
//! - Click events for slot interaction
//! - Visual feedback for hover and selection
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::game_ui::inventory::*;
//!
//! // Create a basic inventory
//! InventoryGridBuilder::new(6, 4)
//!     .slot_size(Val::Px(64.0))
//!     .slot_spacing(Val::Px(4.0))
//!     .drag_drop(true)
//!     .build(parent);
//!
//! // Handle slot clicks
//! fn handle_clicks(mut events: MessageReader<SlotClickEvent>) {
//!     for event in events.read() {
//!         println!("Clicked slot {} with {:?}", event.index, event.button);
//!     }
//! }
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{InventoryGridBuilder, inventory_grid};
pub use plugin::InventoryPlugin;
pub use types::{
    InventoryGrid, InventorySlot, InventoryItem, SlotStyle,
    SlotClickEvent, ItemDragStartEvent, ItemDropEvent,
    InventoryDragState, DragInfo, InventorySettings,
};

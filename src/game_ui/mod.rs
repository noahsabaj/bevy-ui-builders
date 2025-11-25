//! Game UI components
//!
//! This module contains UI components specifically designed for game interfaces.
//! These components are feature-gated under `game_ui`.
//!
//! # Components
//!
//! - **inventory** - Grid-based inventory with drag-and-drop
//! - **resource_bar** - Health/mana/stamina bars with animations
//! - **minimap** - Minimap display with markers
//! - **dialogue** - RPG-style dialogue boxes with portraits
//!
//! # Example
//!
//! ```ignore
//! use bevy_ui_builders::game_ui::*;
//!
//! // Create an inventory grid
//! InventoryGridBuilder::new(6, 4)
//!     .slot_size(Val::Px(64.0))
//!     .drag_drop(true)
//!     .build(parent);
//!
//! // Create a health bar
//! ResourceBarBuilder::new()
//!     .value(80.0)
//!     .max_value(100.0)
//!     .style(ResourceBarStyle::Health)
//!     .build(parent);
//! ```

#[cfg(feature = "inventory")]
pub mod inventory;

#[cfg(feature = "resource_bar")]
pub mod resource_bar;

#[cfg(feature = "minimap")]
pub mod minimap;

#[cfg(feature = "dialogue")]
pub mod dialogue;

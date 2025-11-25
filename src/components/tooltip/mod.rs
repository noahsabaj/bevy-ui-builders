//! Tooltip component
//!
//! Provides tooltips that appear when hovering over UI elements.
//!
//! # Features
//!
//! - Simple text tooltips
//! - Rich tooltips with title and description
//! - Configurable delay and position
//! - Automatic cleanup
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::prelude::*;
//!
//! // Attach tooltip to a button
//! ButtonBuilder::new("Settings")
//!     .tooltip("Open application settings")
//!     .build(parent);
//!
//! // Rich tooltip
//! TooltipBuilder::rich("Save", "Save current document to disk")
//!     .position(TooltipPosition::Bottom)
//!     .delay_ms(300)
//!     .build_for(&mut commands, button_entity);
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{TooltipBuilder, tooltip};
pub use plugin::TooltipPlugin;
pub use types::{HasTooltip, TooltipContent, TooltipEntity, TooltipSettings, TooltipState};

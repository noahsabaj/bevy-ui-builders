//! Minimap component
//!
//! Provides a minimap display for game UIs.
//!
//! # Features
//!
//! - Multiple shapes (circle, square, rounded)
//! - Player rotation following
//! - Zoom control
//! - Marker support
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::game_ui::minimap::*;
//!
//! // Create a circular minimap
//! MinimapBuilder::new()
//!     .size(200.0)
//!     .shape(MinimapShape::Circle)
//!     .rotation(MinimapRotation::FollowPlayer)
//!     .zoom(1.5)
//!     .build(parent);
//! ```

mod builder;
mod plugin;
mod types;

pub use builder::{MinimapBuilder, minimap};
pub use plugin::MinimapPlugin;
pub use types::{
    Minimap, MinimapShape, MinimapRotation, MinimapContent,
    MinimapMarker, MarkerType, MinimapSettings,
};

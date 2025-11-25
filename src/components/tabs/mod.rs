//! Tab view component
//!
//! Provides tabbed interfaces with compile-time safety using the type-state pattern.
//!
//! # Type-State Pattern
//!
//! The builder uses type-states to ensure at least one tab exists before building:
//!
//! ```ignore
//! // This compiles - has at least one tab
//! TabViewBuilder::new()
//!     .tab("Settings", |content| { /* ... */ })
//!     .build(parent);
//!
//! // This won't compile - no tabs!
//! // TabViewBuilder::new().build(parent);  // ERROR: `build` not available
//! ```
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::prelude::*;
//!
//! TabViewBuilder::new()
//!     .style(TabStyle::Pills)
//!     .position(TabPosition::Top)
//!     .tab("General", |content| {
//!         LabelBuilder::new("General settings").build(content);
//!     })
//!     .tab("Advanced", |content| {
//!         LabelBuilder::new("Advanced settings").build(content);
//!     })
//!     .tab_with_config(
//!         TabConfig::new("Notifications").badge(3),
//!         |content| {
//!             LabelBuilder::new("You have notifications").build(content);
//!         }
//!     )
//!     .active(0)
//!     .build(parent);
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{TabViewBuilder, NoTabs, HasTabs, tabs};
pub use plugin::TabsPlugin;
pub use types::{
    TabView, TabButton, TabContent, TabPosition, TabStyle,
    TabSelectedEvent, TabConfig,
};

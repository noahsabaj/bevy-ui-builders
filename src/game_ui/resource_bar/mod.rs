//! Resource bar component
//!
//! Provides animated resource bars for health, mana, stamina, etc.
//!
//! # Features
//!
//! - Multiple style presets (health, mana, stamina, experience)
//! - Smooth fill animations
//! - Damage indicator that trails behind health loss
//! - Customizable colors and dimensions
//! - Optional text overlay
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::game_ui::resource_bar::*;
//!
//! // Create a health bar
//! ResourceBarBuilder::new()
//!     .value(80.0)
//!     .max_value(100.0)
//!     .style(ResourceBarStyle::Health)
//!     .animated(true)
//!     .build(parent);
//!
//! // Create a mana bar with text
//! mana_bar()
//!     .value(50.0)
//!     .max_value(100.0)
//!     .show_text(true)
//!     .build(parent);
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{
    ResourceBarBuilder, health_bar, mana_bar, stamina_bar, experience_bar,
};
pub use plugin::ResourceBarPlugin;
pub use types::{
    ResourceBar, ResourceBarFill, ResourceBarDamageIndicator,
    ResourceBarStyle, ResourceBarConfig, ResourceBarChanged, ResourceBarSettings,
};

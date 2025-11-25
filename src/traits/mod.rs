//! Builder traits for bevy-ui-builders.
//!
//! This module defines the trait hierarchy for all UI builders:
//!
//! - `UiBuilder` - Base trait with `build()`, `insert()`, `with_entity()`
//! - `LayoutBuilder` - Layout properties (width, height, margin, padding, etc.)
//! - `StyleBuilder` - Visual style (variant, colors)
//! - `SizeableBuilder` - Size presets (small, medium, large)
//! - `InteractiveBuilder` - Interaction (disabled, hover effects, animations)
//! - `ContentBuilder` - Content helpers (tooltip)
//! - `LabeledBuilder` - Text content (label, placeholder)
//! - `IconBuilder` - Icon support
//!
//! # Naming Conventions
//!
//! All builder methods follow consistent naming:
//!
//! - **No `with_` prefix** for property setters: `.width()`, `.height()`, `.margin()`
//! - **Chainable**: All methods return `Self` for fluent API
//! - **Optional by default**: Properties have sensible defaults
//!
//! # Example
//!
//! ```ignore
//! use bevy_ui_builders::traits::*;
//!
//! // A button implements multiple traits
//! ButtonBuilder::new("Click")
//!     // LayoutBuilder
//!     .width(Val::Px(200.0))
//!     .margin(UiRect::all(Val::Px(8.0)))
//!     // StyleBuilder
//!     .variant(ButtonVariant::Primary)
//!     // SizeableBuilder
//!     .size(ButtonSize::Large)
//!     // InteractiveBuilder
//!     .on_hover(HoverEffect::Scale(1.05))
//!     // ContentBuilder
//!     .tooltip("Click to submit")
//!     .build(parent);
//! ```

mod content;
mod core;
mod interactive;
mod style;

// Re-export core traits
pub use self::core::{BuilderBase, LayoutBuilder, UiBuilder};

// Re-export style traits and types
pub use self::style::{
    CommonSize, CommonVariant, SizeableBuilder, StyleBuilder, StyleOverrides,
};

// Re-export interactive traits and types
pub use self::interactive::{
    Disabled, InteractiveBuilder, InteractiveConfig, InteractiveState,
};

// Re-export content traits and types
pub use self::content::{
    ContentBuilder, HelpTextBuilder, IconBuilder, IconPosition, LabeledBuilder, TooltipConfig,
    TooltipPosition,
};

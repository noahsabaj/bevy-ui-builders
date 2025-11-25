//! UI Component builders.
//!
//! This module contains all the UI component builders organized by type:
//!
//! - **button** - Interactive buttons with variants and sizes
//! - **checkbox** - Toggle checkboxes
//! - **context_menu** - Right-click context menus
//! - **dialog** - Modal dialogs with overlays
//! - **dropdown** - Dropdown select menus
//! - **form** - Form builder with validation
//! - **label** - Text labels with styles
//! - **number_input** - Numeric input fields
//! - **panel** - Container panels
//! - **progress** - Progress bars
//! - **scroll_view** - Scrollable containers
//! - **separator** - Visual dividers
//! - **slider** - Value sliders
//! - **tabs** - Tabbed interfaces
//! - **text_input** - Text input fields
//! - **toast** - Toast notifications
//! - **tooltip** - Hover tooltips
//!
//! # Example
//!
//! ```ignore
//! use bevy_ui_builders::components::button::ButtonBuilder;
//!
//! ButtonBuilder::new("Click Me")
//!     .style(ButtonStyle::Primary)
//!     .build(parent);
//! ```

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "checkbox")]
pub mod checkbox;

#[cfg(feature = "context_menu")]
pub mod context_menu;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(feature = "dropdown")]
pub mod dropdown;

#[cfg(feature = "form")]
pub mod form;

#[cfg(feature = "label")]
pub mod label;

#[cfg(feature = "number_input")]
pub mod number_input;

#[cfg(feature = "panel")]
pub mod panel;

#[cfg(feature = "progress")]
pub mod progress;

// scroll_view is always available (core functionality)
pub mod scroll_view;

#[cfg(feature = "separator")]
pub mod separator;

#[cfg(feature = "slider")]
pub mod slider;

#[cfg(feature = "tabs")]
pub mod tabs;

#[cfg(feature = "text_input")]
pub mod text_input;

#[cfg(feature = "toast")]
pub mod toast;

#[cfg(feature = "tooltip")]
pub mod tooltip;

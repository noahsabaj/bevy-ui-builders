//! Toast notification component
//!
//! Provides toast notifications with queue-based management.
//!
//! # Features
//!
//! - Multiple toast variants (info, success, warning, error)
//! - Configurable position and duration
//! - Optional action buttons
//! - Dismissible toasts
//! - Queue management for multiple toasts
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::prelude::*;
//!
//! fn show_notifications(mut toasts: ResMut<ToastQueue>) {
//!     // Simple success toast
//!     ToastBuilder::success("File saved!")
//!         .show(&mut toasts);
//!
//!     // Rich error toast with action
//!     ToastBuilder::error("Upload failed")
//!         .title("Network Error")
//!         .duration_secs(10)
//!         .action("Retry")
//!         .show(&mut toasts);
//!
//!     // Using convenience functions
//!     show_success(&mut toasts, "Operation completed");
//!     show_error(&mut toasts, "Something went wrong");
//! }
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{ToastBuilder, show_toast, show_success, show_error, show_warning};
pub use plugin::ToastPlugin;
pub use types::{
    Toast, ToastVariant, ToastPosition, ToastQueue, ToastSettings,
    ActiveToast, ToastContainer, ToastActionEvent, DismissToastEvent,
};

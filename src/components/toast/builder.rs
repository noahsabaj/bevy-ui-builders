//! ToastBuilder implementation

use bevy::prelude::*;
use std::time::Duration;
use super::types::*;

/// Builder for creating toast notifications
///
/// Toasts are shown using a queue-based system.
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// fn show_notification(mut toasts: ResMut<ToastQueue>) {
///     // Simple toast
///     ToastBuilder::new("File saved!")
///         .variant(ToastVariant::Success)
///         .show(&mut toasts);
///
///     // Rich toast
///     ToastBuilder::new("Connection lost")
///         .title("Network Error")
///         .variant(ToastVariant::Error)
///         .duration_secs(10)
///         .action("Retry")
///         .show(&mut toasts);
/// }
/// ```
pub struct ToastBuilder {
    toast: Toast,
}

impl ToastBuilder {
    /// Create a new toast builder with a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            toast: Toast::new(message),
        }
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            toast: Toast::success(message),
        }
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            toast: Toast::error(message),
        }
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            toast: Toast::warning(message),
        }
    }

    /// Create an info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            toast: Toast {
                variant: ToastVariant::Info,
                ..Toast::new(message)
            },
        }
    }

    /// Set the toast title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.toast.title = Some(title.into());
        self
    }

    /// Set the toast variant
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.toast.variant = variant;
        self
    }

    /// Set the display duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.toast.duration = duration;
        self
    }

    /// Set the display duration in seconds
    pub fn duration_secs(mut self, secs: u64) -> Self {
        self.toast.duration = Duration::from_secs(secs);
        self
    }

    /// Set the display duration in milliseconds
    pub fn duration_ms(mut self, ms: u64) -> Self {
        self.toast.duration = Duration::from_millis(ms);
        self
    }

    /// Set whether the toast is dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.toast.dismissible = dismissible;
        self
    }

    /// Add an action button
    pub fn action(mut self, text: impl Into<String>) -> Self {
        self.toast.action = Some(text.into());
        self
    }

    /// Show the toast by adding it to the queue
    pub fn show(self, queue: &mut ToastQueue) {
        queue.push(self.toast);
    }

    /// Get the built toast (for manual handling)
    pub fn build(self) -> Toast {
        self.toast
    }
}

/// Convenience function to show a simple toast
pub fn show_toast(queue: &mut ToastQueue, message: impl Into<String>) {
    ToastBuilder::new(message).show(queue);
}

/// Convenience function to show a success toast
pub fn show_success(queue: &mut ToastQueue, message: impl Into<String>) {
    ToastBuilder::success(message).show(queue);
}

/// Convenience function to show an error toast
pub fn show_error(queue: &mut ToastQueue, message: impl Into<String>) {
    ToastBuilder::error(message).show(queue);
}

/// Convenience function to show a warning toast
pub fn show_warning(queue: &mut ToastQueue, message: impl Into<String>) {
    ToastBuilder::warning(message).show(queue);
}

//! Toast types and components

use bevy::prelude::*;
use std::time::Duration;
use std::collections::VecDeque;

use crate::theme::UiTheme;

/// Toast notification variant (determines color/icon)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    /// Informational toast (blue)
    #[default]
    Info,
    /// Success toast (green)
    Success,
    /// Warning toast (yellow)
    Warning,
    /// Error toast (red)
    Error,
}

/// Position for toast notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    /// Top-left corner
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}

/// A single toast notification
#[derive(Clone, Debug)]
pub struct Toast {
    /// The message to display
    pub message: String,
    /// Optional title
    pub title: Option<String>,
    /// Toast variant (type)
    pub variant: ToastVariant,
    /// How long to show the toast
    pub duration: Duration,
    /// Whether the toast can be dismissed by clicking
    pub dismissible: bool,
    /// Optional action button text
    pub action: Option<String>,
}

impl Toast {
    /// Create a new toast with just a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::Info,
            duration: Duration::from_secs(5),
            dismissible: true,
            action: None,
        }
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::Success,
            duration: Duration::from_secs(3),
            dismissible: true,
            action: None,
        }
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::Error,
            duration: Duration::from_secs(8),
            dismissible: true,
            action: None,
        }
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::Warning,
            duration: Duration::from_secs(5),
            dismissible: true,
            action: None,
        }
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set duration in seconds
    pub fn duration_secs(mut self, secs: u64) -> Self {
        self.duration = Duration::from_secs(secs);
        self
    }

    /// Set whether dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Add an action button
    pub fn action(mut self, text: impl Into<String>) -> Self {
        self.action = Some(text.into());
        self
    }
}

/// Queue of pending toasts to show
#[derive(Resource, Default)]
pub struct ToastQueue {
    /// Pending toasts to display
    pub queue: VecDeque<Toast>,
    /// Maximum number of visible toasts
    pub max_visible: usize,
}

impl ToastQueue {
    /// Create a new toast queue
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_visible: 5,
        }
    }

    /// Add a toast to the queue
    pub fn push(&mut self, toast: Toast) {
        self.queue.push_back(toast);
    }

    /// Pop the next toast from the queue
    pub fn pop(&mut self) -> Option<Toast> {
        self.queue.pop_front()
    }

    /// Check if there are pending toasts
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty()
    }
}

/// Component marking an active toast entity
#[derive(Component, Clone, Debug)]
pub struct ActiveToast {
    /// The toast data
    pub toast: Toast,
    /// Time remaining until auto-dismiss
    pub time_remaining: f32,
    /// Animation progress (0.0 to 1.0 for enter, then stays at 1.0)
    pub animation_progress: f32,
    /// Whether the toast is exiting
    pub exiting: bool,
}

/// Component for the toast container
#[derive(Component, Clone, Debug)]
pub struct ToastContainer {
    /// Position of the container
    pub position: ToastPosition,
}

/// Global toast settings
#[derive(Resource, Clone, Debug)]
pub struct ToastSettings {
    /// Default position for toasts
    pub position: ToastPosition,
    /// Maximum number of visible toasts
    pub max_visible: usize,
    /// Gap between toasts
    pub gap: f32,
    /// Toast width
    pub width: f32,
    /// Z-index for toasts
    pub z_index: i32,
    /// Animation duration
    pub animation_duration: f32,
}

impl Default for ToastSettings {
    fn default() -> Self {
        Self {
            position: ToastPosition::TopRight,
            max_visible: 5,
            gap: 8.0,
            width: 350.0,
            z_index: 3000,
            animation_duration: 0.2,
        }
    }
}

/// Message emitted when a toast action button is clicked
#[derive(Message, Clone, Debug)]
pub struct ToastActionEvent {
    /// The toast entity
    pub entity: Entity,
    /// The action text
    pub action: String,
}

/// Message to dismiss a specific toast
#[derive(Message, Clone, Debug)]
pub struct DismissToastEvent {
    /// The toast entity to dismiss
    pub entity: Entity,
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.12);
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);
    pub const WARNING: Color = Color::srgb(0.96, 0.76, 0.05);
    pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const TEXT_ON_PRIMARY: Color = Color::WHITE;
}

/// Resolved toast colors from theme
#[derive(Clone)]
pub struct ToastColors {
    /// Background color for toast container
    pub background: Color,
    /// Info variant accent color
    pub info: Color,
    /// Success variant accent color
    pub success: Color,
    /// Warning variant accent color
    pub warning: Color,
    /// Error variant accent color
    pub error: Color,
    /// Primary text color
    pub text_primary: Color,
    /// Secondary text color
    pub text_secondary: Color,
    /// Text on action button
    pub text_on_button: Color,
}

impl ToastColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        Self {
            background: theme.colors.surface.tertiary,
            info: theme.colors.primary.base,
            success: theme.colors.success.base,
            warning: theme.colors.warning.base,
            error: theme.colors.danger.base,
            text_primary: theme.colors.text.primary,
            text_secondary: theme.colors.text.secondary,
            text_on_button: theme.colors.primary.on_color,
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            background: defaults::BACKGROUND,
            info: defaults::PRIMARY,
            success: defaults::SUCCESS,
            warning: defaults::WARNING,
            error: defaults::DANGER,
            text_primary: defaults::TEXT_PRIMARY,
            text_secondary: defaults::TEXT_SECONDARY,
            text_on_button: defaults::TEXT_ON_PRIMARY,
        }
    }

    /// Get the accent color for a variant
    pub fn accent_for_variant(&self, variant: ToastVariant) -> Color {
        match variant {
            ToastVariant::Info => self.info,
            ToastVariant::Success => self.success,
            ToastVariant::Warning => self.warning,
            ToastVariant::Error => self.error,
        }
    }
}

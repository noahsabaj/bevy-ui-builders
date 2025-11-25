//! Content-related builder traits.

use super::UiBuilder;

/// Trait for builders that can have tooltip content.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("Hover me")
///     .tooltip("This is helpful information")
///     .build(parent);
/// ```
pub trait ContentBuilder: UiBuilder {
    /// Add a simple text tooltip
    fn tooltip(self, text: impl Into<String>) -> Self;

    /// Add a tooltip with a title and description
    fn tooltip_with_title(self, title: impl Into<String>, description: impl Into<String>) -> Self;
}

/// Configuration for tooltip content.
#[derive(Debug, Clone, Default)]
pub struct TooltipConfig {
    /// Simple tooltip text
    pub text: Option<String>,
    /// Tooltip title (for rich tooltips)
    pub title: Option<String>,
    /// Tooltip description (for rich tooltips)
    pub description: Option<String>,
    /// Tooltip position preference
    pub position: TooltipPosition,
    /// Delay before showing tooltip (in seconds)
    pub delay: f32,
    /// Maximum width of tooltip
    pub max_width: Option<f32>,
}

impl TooltipConfig {
    /// Create a simple text tooltip
    pub fn simple(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            delay: 0.5, // Default 500ms delay
            ..Self::default()
        }
    }

    /// Create a rich tooltip with title and description
    pub fn rich(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            description: Some(description.into()),
            delay: 0.5,
            ..Self::default()
        }
    }

    /// Check if tooltip is configured
    pub fn is_configured(&self) -> bool {
        self.text.is_some() || self.title.is_some()
    }

    /// Set the delay before showing
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// Set the position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set maximum width
    pub fn with_max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }
}

/// Preferred tooltip position relative to the target element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    /// Above the element
    #[default]
    Top,
    /// Below the element
    Bottom,
    /// To the left of the element
    Left,
    /// To the right of the element
    Right,
    /// Automatically choose based on available space
    Auto,
}

/// Trait for builders that support labels/text content.
pub trait LabeledBuilder: UiBuilder {
    /// Set the label text
    fn label(self, text: impl Into<String>) -> Self;

    /// Set the placeholder text (for inputs)
    fn placeholder(self, text: impl Into<String>) -> Self;
}

/// Trait for builders that support icons.
pub trait IconBuilder: UiBuilder {
    /// Set an icon by name/path
    fn icon(self, icon: impl Into<String>) -> Self;

    /// Set icon position relative to text
    fn icon_position(self, position: IconPosition) -> Self;
}

/// Position of icon relative to text content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPosition {
    /// Icon before text (left in LTR)
    #[default]
    Start,
    /// Icon after text (right in LTR)
    End,
    /// Icon only, no text
    Only,
}

/// Trait for builders that support help text.
pub trait HelpTextBuilder: UiBuilder {
    /// Add help text below the element
    fn help_text(self, text: impl Into<String>) -> Self;

    /// Add error message text
    fn error_text(self, text: impl Into<String>) -> Self;
}

//! Style-related builder traits.

use bevy::prelude::*;

use super::UiBuilder;

/// Trait for builders that support style variants.
///
/// This provides a consistent API for setting visual appearance
/// across different component types.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("Click")
///     .variant(ButtonVariant::Primary)
///     .build(parent);
/// ```
pub trait StyleBuilder: UiBuilder {
    /// The variant enum type for this builder
    type Variant: Default + Clone;

    /// Set the style variant
    fn variant(self, variant: Self::Variant) -> Self;

    /// Override the background color (ignores variant)
    fn background(self, color: Color) -> Self;

    /// Override the text/foreground color
    fn text_color(self, color: Color) -> Self;

    /// Override the border color
    fn border_color(self, color: Color) -> Self;
}

/// Trait for builders that support size presets.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("Click")
///     .size(ButtonSize::Large)
///     .build(parent);
/// ```
pub trait SizeableBuilder: UiBuilder {
    /// The size enum type for this builder
    type Size: Default + Clone;

    /// Set the size preset
    fn size(self, size: Self::Size) -> Self;
}

/// Common style variants that can be used across components.
///
/// Individual components may define their own variant enums,
/// but this provides a standard set for consistency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommonVariant {
    /// Primary/default appearance
    #[default]
    Primary,
    /// Secondary/less prominent appearance
    Secondary,
    /// Success/positive state
    Success,
    /// Warning/cautionary state
    Warning,
    /// Danger/destructive state
    Danger,
    /// Ghost/minimal appearance (transparent background)
    Ghost,
}

/// Common size presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommonSize {
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium (default)
    #[default]
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
}

/// Style override container for components.
///
/// This allows individual style properties to be overridden
/// while still using variant-based styling for others.
#[derive(Debug, Clone, Default)]
pub struct StyleOverrides {
    /// Override background color
    pub background: Option<Color>,
    /// Override text/foreground color
    pub text_color: Option<Color>,
    /// Override border color
    pub border_color: Option<Color>,
    /// Override border width
    pub border_width: Option<f32>,
    /// Override border radius
    pub border_radius: Option<f32>,
}

impl StyleOverrides {
    /// Create empty overrides
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any overrides are set
    pub fn has_overrides(&self) -> bool {
        self.background.is_some()
            || self.text_color.is_some()
            || self.border_color.is_some()
            || self.border_width.is_some()
            || self.border_radius.is_some()
    }

    /// Set background override
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set text color override
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set border color override
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}

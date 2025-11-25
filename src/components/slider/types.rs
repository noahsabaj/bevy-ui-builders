//! Slider types and components

use bevy::prelude::*;
use bevy::color::Alpha;

use crate::theme::{UiTheme, SemanticVariant};

/// Main slider component with configuration and state
#[derive(Component, Clone, Debug)]
pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    /// Entity ID of the associated value text display (if any)
    pub value_text_entity: Option<Entity>,
}

impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            step: None,
            value_text_entity: None,
        }
    }

    /// Get normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f32 {
        if self.max == self.min {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    /// Set value from normalized (0.0 to 1.0)
    pub fn set_normalized(&mut self, normalized: f32) {
        let normalized = normalized.clamp(0.0, 1.0);
        self.value = self.min + (self.max - self.min) * normalized;

        // Apply step if configured
        if let Some(step) = self.step {
            let steps = ((self.value - self.min) / step).round();
            self.value = self.min + steps * step;
        }

        self.value = self.value.clamp(self.min, self.max);
    }
}

/// Marker for the draggable handle
#[derive(Component)]
pub struct SliderHandle;

/// Component for increment/decrement buttons
#[derive(Component)]
pub struct SliderButtonAction {
    pub slider_entity: Entity,
    pub delta: f32,
}

/// Component for the slider track (clickable area)
#[derive(Component)]
pub struct SliderTrack;

/// Component for the filled portion of the slider
#[derive(Component)]
pub struct SliderFill;

/// Component for the value text display
#[derive(Component)]
pub struct SliderValueText;

/// Component for the label text
#[derive(Component)]
pub struct SliderLabel;

/// Configuration for how the value is displayed
#[derive(Clone, Debug)]
pub enum ValueFormat {
    /// Display as integer
    Integer,
    /// Display with fixed decimal places
    Decimal(usize),
    /// Display as percentage
    Percentage,
    /// Custom formatter function
    Custom(fn(f32) -> String),
}

impl ValueFormat {
    /// Format a value according to this format
    pub fn format(&self, value: f32) -> String {
        match self {
            ValueFormat::Integer => format!("{}", value as i32),
            ValueFormat::Decimal(places) => format!("{:.precision$}", value, precision = places),
            ValueFormat::Percentage => format!("{}%", (value * 100.0) as i32),
            ValueFormat::Custom(formatter) => formatter(value),
        }
    }
}

/// Configuration for slider appearance
#[derive(Component, Clone, Debug)]
pub struct SliderConfig {
    pub show_value: bool,
    pub value_format: ValueFormat,
    pub track_height: f32,
    pub handle_size: f32,
    pub track_color: Color,
    pub fill_color: Color,
    pub handle_color: Color,
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;
    use bevy::color::Alpha;

    pub const TRACK_COLOR: Color = Color::srgb(0.12, 0.12, 0.14);
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub fn fill_color() -> Color { PRIMARY.with_alpha(0.3) }
    pub const HANDLE_COLOR: Color = PRIMARY;
    pub const HANDLE_HOVER: Color = Color::srgb(0.35, 0.56, 0.96);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const BORDER_LIGHT: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const BORDER_FOCUS: Color = PRIMARY;
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            show_value: true,
            value_format: ValueFormat::Decimal(1),
            track_height: 6.0,
            handle_size: 16.0,
            track_color: defaults::TRACK_COLOR,
            fill_color: defaults::fill_color(),
            handle_color: defaults::HANDLE_COLOR,
        }
    }
}

/// Resolved slider colors from theme
#[derive(Clone)]
pub struct SliderColors {
    pub track: Color,
    pub fill: Color,
    pub handle: Color,
    pub handle_hover: Color,
    pub handle_border: Color,
    pub handle_border_hover: Color,
    pub text_label: Color,
    pub text_value: Color,
}

impl SliderColors {
    /// Resolve colors from theme based on variant
    pub fn from_theme(theme: &UiTheme, variant: SemanticVariant) -> Self {
        let base_color = match variant {
            SemanticVariant::Primary => theme.colors.primary.base,
            SemanticVariant::Secondary => theme.colors.secondary.base,
            SemanticVariant::Success => theme.colors.success.base,
            SemanticVariant::Warning => theme.colors.warning.base,
            SemanticVariant::Danger => theme.colors.danger.base,
            SemanticVariant::Ghost => theme.colors.text.secondary,
        };
        let hover_color = match variant {
            SemanticVariant::Primary => theme.colors.primary.hover,
            SemanticVariant::Secondary => theme.colors.secondary.hover,
            SemanticVariant::Success => theme.colors.success.hover,
            SemanticVariant::Warning => theme.colors.warning.hover,
            SemanticVariant::Danger => theme.colors.danger.hover,
            SemanticVariant::Ghost => theme.colors.text.primary,
        };

        Self {
            track: theme.colors.surface.tertiary,
            fill: base_color.with_alpha(0.3),
            handle: base_color,
            handle_hover: hover_color,
            handle_border: theme.colors.border.light,
            handle_border_hover: theme.colors.border.focus,
            text_label: theme.colors.text.secondary,
            text_value: theme.colors.text.primary,
        }
    }

    /// Default colors (no theme)
    pub fn default_colors(variant: SemanticVariant) -> Self {
        let (base_color, hover_color) = match variant {
            SemanticVariant::Primary | SemanticVariant::Ghost => (defaults::PRIMARY, defaults::HANDLE_HOVER),
            SemanticVariant::Secondary => (Color::srgb(0.25, 0.25, 0.25), Color::srgb(0.35, 0.35, 0.35)),
            SemanticVariant::Success => (Color::srgb(0.25, 0.76, 0.25), Color::srgb(0.35, 0.86, 0.35)),
            SemanticVariant::Warning => (Color::srgb(0.96, 0.76, 0.05), Color::srgb(1.0, 0.86, 0.15)),
            SemanticVariant::Danger => (Color::srgb(0.86, 0.25, 0.25), Color::srgb(0.96, 0.35, 0.35)),
        };

        Self {
            track: defaults::TRACK_COLOR,
            fill: base_color.with_alpha(0.3),
            handle: base_color,
            handle_hover: hover_color,
            handle_border: defaults::BORDER_LIGHT,
            handle_border_hover: base_color,
            text_label: defaults::TEXT_SECONDARY,
            text_value: defaults::TEXT_PRIMARY,
        }
    }
}
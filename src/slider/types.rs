//! Slider types and components

use bevy::prelude::*;
use crate::styles::colors;

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

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            show_value: true,
            value_format: ValueFormat::Decimal(1),
            track_height: 6.0,
            handle_size: 16.0,
            track_color: colors::BACKGROUND_TERTIARY,
            fill_color: colors::PRIMARY.with_alpha(0.3),
            handle_color: colors::PRIMARY,
        }
    }
}
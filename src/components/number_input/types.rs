//! Number input component types

use bevy::prelude::*;

/// Marker component for number input fields
#[derive(Component, Debug, Clone, Copy)]
pub struct NumberInput;

/// Configuration for number input validation
#[derive(Component, Debug, Clone)]
pub struct NumberInputConfig {
    /// Minimum allowed value
    pub min: Option<f32>,
    /// Maximum allowed value
    pub max: Option<f32>,
    /// Step size for increment/decrement (future: +/- buttons)
    pub step: f32,
}

impl Default for NumberInputConfig {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            step: 1.0,
        }
    }
}

impl NumberInputConfig {
    /// Clamp a value to the configured min/max range
    pub fn clamp_value(&self, value: f32) -> f32 {
        let mut result = value;
        if let Some(min) = self.min {
            result = result.max(min);
        }
        if let Some(max) = self.max {
            result = result.min(max);
        }
        result
    }

    /// Validate if a value is within the configured range
    pub fn is_valid(&self, value: f32) -> bool {
        if let Some(min) = self.min {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if value > max {
                return false;
            }
        }
        true
    }
}

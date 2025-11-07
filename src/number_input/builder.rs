//! NumberInputBuilder for creating validated number input fields

use bevy::prelude::*;
use super::types::*;
use crate::text_input::{TextInputBuilder, InputFilter};

/// Builder for creating number input fields with validation
///
/// # Examples
///
/// ```rust
/// use bevy_ui_builders::prelude::*;
///
/// fn build_number_input(parent: &mut ChildSpawnerCommands) {
///     NumberInputBuilder::new()
///         .min(8.0)
///         .max(24.0)
///         .default_value(16.0)
///         .build(parent);
/// }
/// ```
pub struct NumberInputBuilder {
    min: Option<f32>,
    max: Option<f32>,
    step: f32,
    default_value: Option<f32>,
    width: Val,
    placeholder: Option<String>,
}

impl NumberInputBuilder {
    /// Create a new number input builder
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            step: 1.0,
            default_value: None,
            width: Val::Px(crate::styles::dimensions::INPUT_WIDTH_DEFAULT),
            placeholder: None,
        }
    }

    /// Set the minimum allowed value
    pub fn min(mut self, min: f32) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum allowed value
    pub fn max(mut self, max: f32) -> Self {
        self.max = Some(max);
        self
    }

    /// Set the step size for increment/decrement
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set the default value
    pub fn default_value(mut self, value: f32) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Set the width of the input field
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Build the number input and spawn it
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let config = NumberInputConfig {
            min: self.min,
            max: self.max,
            step: self.step,
        };

        // Clamp default value to range if provided
        let initial_value = if let Some(value) = self.default_value {
            Some(config.clamp_value(value).to_string())
        } else {
            None
        };

        // Build hint text for min/max if provided
        let hint = match (self.min, self.max) {
            (Some(min), Some(max)) => Some(format!("Range: {}-{}", min, max)),
            (Some(min), None) => Some(format!("Min: {}", min)),
            (None, Some(max)) => Some(format!("Max: {}", max)),
            (None, None) => None,
        };

        // Create the text input with decimal filter
        let mut text_input = TextInputBuilder::new()
            .with_width(self.width)
            .with_filter(InputFilter::Decimal);

        // Set placeholder or hint
        if let Some(placeholder) = self.placeholder {
            text_input = text_input.with_placeholder(&placeholder);
        } else if let Some(hint_text) = hint {
            text_input = text_input.with_placeholder(&hint_text);
        }

        // Set initial value if provided
        if let Some(value_str) = initial_value {
            text_input = text_input.with_value(&value_str);
        }

        // Add automatic validation for range if min or max is specified
        if self.min.is_some() || self.max.is_some() {
            let min = self.min.unwrap_or(f32::MIN);
            let max = self.max.unwrap_or(f32::MAX);
            text_input = text_input.with_validation(vec![
                crate::ValidationRule::Range { min, max }
            ]);
        }

        // Build and add marker components
        let entity = text_input.build(parent);

        parent.commands().entity(entity).insert((
            NumberInput,
            config,
        ));

        entity
    }
}

impl Default for NumberInputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

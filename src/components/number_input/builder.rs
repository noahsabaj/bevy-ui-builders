//! NumberInputBuilder for creating validated number input fields

use bevy::prelude::*;
use super::types::*;
use crate::components::text_input::{TextInputBuilder, InputFilter};
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

/// Builder for creating number input fields with validation
///
/// # Examples
///
/// ```ignore
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
    placeholder: Option<String>,
    base: BuilderBase,
}

impl NumberInputBuilder {
    /// Create a new number input builder
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Px(crate::styles::dimensions::INPUT_WIDTH_DEFAULT);

        Self {
            min: None,
            max: None,
            step: 1.0,
            default_value: None,
            placeholder: None,
            base,
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
        self.base.node.width = width;
        self
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Build the number input and spawn it (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for NumberInputBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
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
        // We initialize it with our base.node to transfer layout properties
        let mut text_input = TextInputBuilder::new()
            .node(self.base.node)
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

        // Build the text input
        let entity = text_input.build(parent);

        // Add NumberInput specific components
        parent.commands().entity(entity).insert((
            NumberInput,
            config,
        ));

        // Apply hooks from NumberInputBuilder
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(entity));
        }

        entity
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for NumberInputBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }
}

impl Default for NumberInputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

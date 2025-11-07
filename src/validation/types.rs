//! Validation component types

use bevy::prelude::*;
use crate::ValidationRule;

/// Marker component - attach to any input that needs validation
#[derive(Component, Debug, Clone)]
pub struct Validated {
    /// Validation rules to apply
    pub rules: Vec<ValidationRule>,
}

impl Validated {
    /// Create a new validated component with rules
    pub fn new(rules: Vec<ValidationRule>) -> Self {
        Self { rules }
    }
}

/// Current validation state for an input
#[derive(Component, Debug, Clone)]
pub struct ValidationState {
    /// Whether the current value is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error_message: Option<String>,
}

impl Default for ValidationState {
    fn default() -> Self {
        Self {
            is_valid: true,
            error_message: None,
        }
    }
}

impl ValidationState {
    /// Create a new valid state
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            error_message: None,
        }
    }

    /// Create a new invalid state with error message
    pub fn invalid(message: String) -> Self {
        Self {
            is_valid: false,
            error_message: Some(message),
        }
    }
}

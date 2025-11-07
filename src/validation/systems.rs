//! Validation systems

use bevy::prelude::*;
use super::types::*;
use crate::text_input::native_input::TextBuffer;
use crate::styles::colors;

/// Validate text inputs when their buffer changes
pub fn validate_text_inputs(
    mut inputs: Query<
        (
            &Validated,
            &mut ValidationState,
            &mut BorderColor,
            &TextBuffer,
        ),
        Changed<TextBuffer>
    >,
) {
    for (validated, mut state, mut border, buffer) in inputs.iter_mut() {
        let value = &buffer.content;

        // Run all validation rules
        let mut errors = Vec::new();
        for rule in &validated.rules {
            if let Err(msg) = rule.validate(value) {
                errors.push(msg);
                break; // Show only first error
            }
        }

        // Update validation state
        if errors.is_empty() {
            *state = ValidationState::valid();
            *border = BorderColor::all(colors::BORDER_DEFAULT);
        } else {
            *state = ValidationState::invalid(errors[0].clone());
            *border = BorderColor::all(colors::BORDER_ERROR);
        }
    }
}

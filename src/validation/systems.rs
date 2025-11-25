//! Validation systems

use bevy::prelude::*;
use super::types::*;
use crate::components::text_input::native_input::TextBuffer;
use crate::theme::UiTheme;

/// Default colors for validation (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BORDER_ERROR: Color = Color::srgb(0.86, 0.25, 0.25);
}

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
    theme: Option<Res<UiTheme>>,
) {
    // Resolve colors from theme or use defaults
    let (border_default, border_error) = if let Some(ref theme) = theme {
        (theme.colors.border.default, theme.colors.danger.base)
    } else {
        (defaults::BORDER_DEFAULT, defaults::BORDER_ERROR)
    };

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
            *border = BorderColor::all(border_default);
        } else {
            *state = ValidationState::invalid(errors[0].clone());
            *border = BorderColor::all(border_error);
        }
    }
}

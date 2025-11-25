//! Focus visual state management

use bevy::prelude::*;
use crate::theme::UiTheme;
use super::super::components::*;

/// Default colors for focus visual (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BORDER_FOCUS: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
}

/// Maintain visual focus state (blue border when focused)
pub fn update_focus_visual(
    mut text_inputs: Query<
        (&TextBuffer, &mut BorderColor),
        (With<NativeTextInput>, Or<(Changed<TextBuffer>, Changed<BorderColor>)>)
    >,
    theme: Option<Res<UiTheme>>,
) {
    // Resolve colors from theme or use defaults
    let (border_focus, border_default) = if let Some(ref theme) = theme {
        (theme.colors.border.focus, theme.colors.border.default)
    } else {
        (defaults::BORDER_FOCUS, defaults::BORDER_DEFAULT)
    };

    for (buffer, mut border_color) in text_inputs.iter_mut() {
        // Apply focus border color when focused, normal border when not
        *border_color = BorderColor::all(if buffer.is_focused {
            border_focus  // Blue border when focused
        } else {
            border_default  // Default border when not focused
        });
    }
}
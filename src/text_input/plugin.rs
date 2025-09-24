//! Text input plugin

use bevy::prelude::*;
use super::systems::*;

/// Plugin that provides the complete text input system
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_text_input_focus,
                handle_click_outside_unfocus,
                validate_text_input_changes,
                handle_clear_button_clicks,
            ),
        );
    }
}
//! Systems for checkbox interaction and visual updates

use bevy::prelude::*;
use super::types::*;

/// Handle checkbox clicks to toggle state
pub fn handle_checkbox_toggle(
    mut checkboxes: Query<(&Interaction, &mut CheckboxState), (With<Checkbox>, Changed<Interaction>)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    for (interaction, mut state) in checkboxes.iter_mut() {
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            state.toggle();
        }
    }
}

/// Update checkbox visual appearance when state changes
pub fn update_checkbox_visuals(
    mut checkboxes: Query<(&CheckboxState, &CheckboxStyleComponent, &mut BackgroundColor, &Children), (With<Checkbox>, Changed<CheckboxState>)>,
    mut checkmarks: Query<&mut Node, With<CheckboxCheckmark>>,
) {
    for (state, style_component, mut bg_color, children) in checkboxes.iter_mut() {
        // Update background color
        *bg_color = BackgroundColor(if state.is_checked() {
            style_component.0.checked_color()
        } else {
            style_component.0.unchecked_color()
        });

        // Update checkmark visibility
        for child in children.iter() {
            if let Ok(mut checkmark_node) = checkmarks.get_mut(child) {
                checkmark_node.display = if state.is_checked() {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

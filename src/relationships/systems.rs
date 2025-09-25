//! Systems for managing UI relationships
//!
//! Contains systems that operate on relationship components to provide
//! functionality like exclusive button groups and form validation.

use bevy::prelude::*;
use super::types::*;

/// System to handle exclusive selection within button groups (e.g., radio buttons).
///
/// When a button in a group is clicked, this system automatically deselects
/// all other buttons in the same group, ensuring only one button is selected at a time.
pub fn handle_exclusive_button_groups(
    mut clicked_query: Query<
        (&InButtonGroup, &Interaction, Entity),
        Changed<Interaction>,
    >,
    group_query: Query<&ButtonGroupMembers>,
    mut button_states: Query<&mut BackgroundColor>,
) {
    for (in_group, interaction, clicked_entity) in &mut clicked_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Get all members of this button's group
        if let Ok(members) = group_query.get(in_group.0) {
            // Deselect all other buttons in the group
            for &member_entity in members.iter() {
                if member_entity != clicked_entity {
                    if let Ok(mut bg_color) = button_states.get_mut(member_entity) {
                        // Reset to unselected state
                        // This would be customized based on your button styling
                        bg_color.0 = Color::srgb(0.25, 0.25, 0.25);
                    }
                }
            }

            // Highlight the selected button
            if let Ok(mut bg_color) = button_states.get_mut(clicked_entity) {
                bg_color.0 = Color::srgb(0.25, 0.46, 0.86); // Primary color
            }
        }
    }
}

/// System to validate all fields in a form using relationships.
///
/// Returns a list of tuples containing the form entity and whether it's valid.
/// This is a utility function that would typically be called from other systems.
pub fn validate_form_fields(
    forms: Query<(Entity, &FormFields)>,
    field_query: Query<&Node>, // Would include validation components
) -> Vec<(Entity, bool)> {
    let mut results = Vec::new();

    for (form_entity, fields) in &forms {
        let mut form_valid = true;

        for &field_entity in fields.iter() {
            if field_query.get(field_entity).is_err() {
                form_valid = false;
                break;
            }
            // Additional validation logic would go here
        }

        results.push((form_entity, form_valid));
    }

    results
}

/// System to update all slider parts when slider value changes.
///
/// This system demonstrates how relationships can be used to update
/// all related entities when a parent entity changes.
pub fn update_slider_parts(
    sliders: Query<(&Slider, &SliderParts), Changed<Slider>>,
    mut transforms: Query<&mut Transform>,
) {
    for (slider, parts) in &sliders {
        // Update all parts based on the slider value
        for &part_entity in parts.iter() {
            if let Ok(mut transform) = transforms.get_mut(part_entity) {
                // Update part position/scale based on slider value
                // This is a placeholder - actual implementation would depend on part type
                let normalized = (slider.value - slider.min) / (slider.max - slider.min);
                transform.translation.x = normalized * 100.0;
            }
        }
    }
}
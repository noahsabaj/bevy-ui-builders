//! Text input interaction systems

use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputValue};
use super::types::*;

/// Handle clicking on text inputs to manage focus
pub fn handle_text_input_focus(
    mut commands: Commands,
    interactions: Query<
        (Entity, &Interaction, &TextInputFocus),
        (Changed<Interaction>, With<TextInput>),
    >,
    all_inputs: Query<(Entity, &TextInputFocus), With<TextInput>>,
) {
    for (clicked_entity, interaction, focus_type) in &interactions {
        if *interaction == Interaction::Pressed {
            match focus_type {
                TextInputFocus::Independent => {
                    // Just focus this one input
                    commands
                        .entity(clicked_entity)
                        .insert(TextInputInactive(false));
                }
                TextInputFocus::ExclusiveGroup(group_id) => {
                    // Focus this input, unfocus others in the same group
                    for (entity, other_focus) in &all_inputs {
                        match other_focus {
                            TextInputFocus::ExclusiveGroup(other_group)
                                if other_group == group_id =>
                            {
                                // Same group - manage focus
                                if entity == clicked_entity {
                                    commands.entity(entity).insert(TextInputInactive(false));
                                } else {
                                    commands.entity(entity).insert(TextInputInactive(true));
                                }
                            }
                            _ => {} // Different group or independent - ignore
                        }
                    }
                }
            }
        }
    }
}

/// Unfocus all text inputs when clicking outside any input
pub fn handle_click_outside_unfocus(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    interactions: Query<&Interaction, With<TextInput>>,
    all_inputs: Query<Entity, With<TextInput>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let clicking_on_input = interactions.iter().any(|i| *i != Interaction::None);

        // If not clicking on any input, unfocus all
        if !clicking_on_input {
            for entity in &all_inputs {
                commands.entity(entity).insert(TextInputInactive(true));
            }
        }
    }
}

/// Validate and filter text input changes based on TextInputFilter
pub fn validate_text_input_changes(
    mut text_inputs: Query<
        (&mut TextInputValue, &TextInputFilter),
        (Changed<TextInputValue>, With<TextInput>),
    >,
) {
    for (mut text_value, filter) in &mut text_inputs {
        let current_text = text_value.0.clone();
        let mut modified_text = current_text.clone();

        // Apply filtering
        modified_text = filter.filter_type.filter_string(&modified_text);

        // Apply max length constraint
        if let Some(max_len) = filter.max_length {
            if modified_text.len() > max_len {
                modified_text.truncate(max_len);
            }
        }

        // Apply text transformation
        modified_text = match filter.transform {
            InputTransform::None => modified_text,
            InputTransform::Uppercase => modified_text.to_uppercase(),
            InputTransform::Lowercase => modified_text.to_lowercase(),
            InputTransform::Capitalize => {
                let mut chars = modified_text.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            }
        };

        // Only update if the text changed
        if modified_text != current_text {
            text_value.0 = modified_text;
        }
    }
}

/// Handle clicks on clear buttons
pub fn handle_clear_button_clicks(
    button_query: Query<(&Interaction, &ClearButtonTarget), (Changed<Interaction>, With<Button>)>,
    mut text_inputs: Query<&mut TextInputValue, With<TextInput>>,
) {
    for (interaction, target) in &button_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text_value) = text_inputs.get_mut(target.0) {
                text_value.0.clear();
            }
        }
    }
}
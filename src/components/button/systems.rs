//! Button interaction systems

use bevy::prelude::*;
use super::types::{StyledButton, ButtonStateColors, ButtonAnimationState, SelectableButton, Selected, Active, ButtonSelectionColors, SelectionChanged};
use crate::relationships::{InButtonGroup, ButtonGroupMembers};

/// Comprehensive button interaction system that handles all hover/pressed states
/// This system sets the target values for the animation system to interpolate
pub fn handle_button_interaction(
    mut query: Query<(
        &Interaction,
        &mut ButtonAnimationState,
    ), (Changed<Interaction>, With<StyledButton>)>,
) {
    for (interaction, mut animation) in &mut query {
        match interaction {
            Interaction::Pressed => {
                // Set target for pressed state
                animation.target_scale = 0.98; // More subtle press
                animation.target_color_blend = 1.0; // Full pressed colors
            }
            Interaction::Hovered => {
                // Set target for hover state
                animation.target_scale = 1.0; // Scale is now handled by UiAnimation
                animation.target_color_blend = 0.5; // Hover colors
            }
            Interaction::None => {
                // Set target for normal state
                animation.target_scale = 1.0;
                animation.target_color_blend = 0.0; // Normal colors
            }
        }
    }
}

/// System to smoothly animate button transitions
pub fn animate_button_transitions(
    mut query: Query<(
        &mut ButtonAnimationState,
        &mut Transform,
        &ButtonStateColors,
        &mut BackgroundColor,
        &mut BorderColor,
    ), With<StyledButton>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut animation, mut transform, state_colors, mut bg_color, mut border_color) in &mut query {
        // Smoothly interpolate scale
        let scale_diff = animation.target_scale - animation.current_scale;
        if scale_diff.abs() > 0.001 {
            animation.current_scale += scale_diff * animation.animation_speed * delta;
            transform.scale = Vec3::splat(animation.current_scale);
        }

        // Smoothly interpolate color blend
        let color_diff = animation.target_color_blend - animation.current_color_blend;
        if color_diff.abs() > 0.001 {
            animation.current_color_blend += color_diff * animation.animation_speed * delta;

            // Blend colors based on current blend value
            let t = animation.current_color_blend;

            // Determine target colors based on blend value
            let (target_bg, target_border) = if t > 0.5 {
                // Blending towards pressed
                let pressed_t = (t - 0.5) * 2.0; // Remap 0.5-1.0 to 0.0-1.0
                (
                    lerp_color(state_colors.hover_bg, state_colors.pressed_bg, pressed_t),
                    lerp_color(state_colors.hover_border, state_colors.pressed_border, pressed_t),
                )
            } else {
                // Blending towards hover
                let hover_t = t * 2.0; // Remap 0.0-0.5 to 0.0-1.0
                (
                    lerp_color(state_colors.normal_bg, state_colors.hover_bg, hover_t),
                    lerp_color(state_colors.normal_border, state_colors.hover_border, hover_t),
                )
            };

            bg_color.0 = target_bg;
            *border_color = BorderColor::all(target_border);
        }
    }
}

/// Helper function to lerp between two colors
fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    let from_linear = from.to_linear();
    let to_linear = to.to_linear();

    Color::LinearRgba(LinearRgba {
        red: from_linear.red + (to_linear.red - from_linear.red) * t,
        green: from_linear.green + (to_linear.green - from_linear.green) * t,
        blue: from_linear.blue + (to_linear.blue - from_linear.blue) * t,
        alpha: from_linear.alpha + (to_linear.alpha - from_linear.alpha) * t,
    })
}

// ============================================================================
// Selection State Systems
// ============================================================================

/// System to handle auto-toggle behavior for selectable buttons
/// Toggles the Selected component when a selectable button is clicked
/// NOTE: Buttons in groups are handled by enforce_exclusive_button_groups instead
pub fn auto_toggle_selectable_buttons(
    mut commands: Commands,
    query: Query<
        (Entity, &Interaction, &SelectableButton, Option<&Selected>),
        (Changed<Interaction>, With<StyledButton>, Without<InButtonGroup>),
    >,
    mut events: MessageWriter<SelectionChanged>,
) {
    for (entity, interaction, selectable, selected) in &query {
        if *interaction == Interaction::Pressed && selectable.auto_toggle {
            if selected.is_some() {
                // Deselect
                commands.entity(entity).remove::<Selected>();
                events.write(SelectionChanged {
                    entity,
                    selected: false,
                });
            } else {
                // Select
                commands.entity(entity).insert(Selected);
                events.write(SelectionChanged {
                    entity,
                    selected: true,
                });
            }
        }
    }
}

/// System to enforce exclusive selection within button groups (radio button behavior)
/// When a button in a group is clicked, deselects all other buttons in the group
pub fn enforce_exclusive_button_groups(
    mut commands: Commands,
    clicked_query: Query<
        (Entity, &Interaction, &InButtonGroup),
        (Changed<Interaction>, With<SelectableButton>),
    >,
    group_query: Query<&ButtonGroupMembers>,
    mut events: MessageWriter<SelectionChanged>,
) {
    for (clicked_entity, interaction, in_group) in &clicked_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Get all members of this button's group
        if let Ok(members) = group_query.get(in_group.0) {
            let member_count = members.iter().count();
            info!("[Radio] Button {:?} clicked in group {:?} with {} members",
                  clicked_entity, in_group.0, member_count);

            // Deselect all other buttons in the group
            for &member_entity in members.iter() {
                if member_entity != clicked_entity {
                    info!("  [-] Deselecting button {:?}", member_entity);
                    commands.entity(member_entity).remove::<Selected>();
                    events.write(SelectionChanged {
                        entity: member_entity,
                        selected: false,
                    });
                }
            }

            // Select the clicked button
            info!("  [+] Selecting button {:?}", clicked_entity);
            commands.entity(clicked_entity).insert(Selected);
            events.write(SelectionChanged {
                entity: clicked_entity,
                selected: true,
            });
        } else {
            warn!("Button {:?} is in group {:?} but group has no ButtonGroupMembers component!",
                  clicked_entity, in_group.0);
        }
    }
}

/// System to update button appearance based on selection state
/// Applies the correct color set based on Active > Selected > Normal priority
/// NOTE: Runs every frame for selectable buttons to immediately reflect state changes
pub fn update_selection_appearance(
    mut query: Query<
        (
            &ButtonSelectionColors,
            &mut ButtonStateColors,
            Option<&Active>,
            Option<&Selected>,
        ),
        With<SelectableButton>,
    >,
) {
    for (selection_colors, mut state_colors, active, selected) in &mut query {
        // Determine which color set to use based on priority: Active > Selected > Normal
        let color_set = if active.is_some() {
            &selection_colors.active
        } else if selected.is_some() {
            &selection_colors.selected
        } else {
            &selection_colors.normal
        };

        // Update the ButtonStateColors to use the appropriate set
        state_colors.normal_bg = color_set.normal_bg;
        state_colors.hover_bg = color_set.hover_bg;
        state_colors.pressed_bg = color_set.pressed_bg;
        state_colors.normal_border = color_set.normal_border;
        state_colors.hover_border = color_set.hover_border;
        state_colors.pressed_border = color_set.pressed_border;
    }
}

/// System to immediately apply colors when selection state changes
/// This ensures the button visuals update instantly without waiting for interaction
pub fn apply_selection_colors_immediately(
    mut query: Query<
        (
            &ButtonStateColors,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (
            With<SelectableButton>,
            Or<(Changed<ButtonStateColors>, Changed<Selected>, Changed<Active>)>,
        ),
    >,
) {
    for (state_colors, interaction, mut bg_color, mut border_color) in &mut query {
        // Apply colors based on current interaction state
        match interaction {
            Interaction::Pressed => {
                bg_color.0 = state_colors.pressed_bg;
                *border_color = BorderColor::all(state_colors.pressed_border);
            }
            Interaction::Hovered => {
                bg_color.0 = state_colors.hover_bg;
                *border_color = BorderColor::all(state_colors.hover_border);
            }
            Interaction::None => {
                bg_color.0 = state_colors.normal_bg;
                *border_color = BorderColor::all(state_colors.normal_border);
            }
        }
    }
}
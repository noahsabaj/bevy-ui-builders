//! Button interaction systems

use bevy::prelude::*;
use super::types::{StyledButton, ButtonStateColors, ButtonAnimationState};
use crate::systems::hover::{HoverScale, HoverBrightness, OriginalColors, apply_brightness};

/// System to handle hover scale effects
pub fn handle_hover_scale(
    mut query: Query<(&Interaction, &HoverScale, &mut Transform), Changed<Interaction>>,
) {
    for (interaction, hover_scale, mut transform) in &mut query {
        match interaction {
            Interaction::Hovered => {
                transform.scale = Vec3::splat(hover_scale.0);
            }
            _ => {
                transform.scale = Vec3::ONE;
            }
        }
    }
}

/// System to handle hover brightness effects
pub fn handle_hover_brightness(
    mut query: Query<
        (&Interaction, &HoverBrightness, &OriginalColors, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, hover_brightness, original, mut bg_color, mut border_color) in &mut query {
        match interaction {
            Interaction::Hovered => {
                bg_color.0 = apply_brightness(original.background, hover_brightness.0);
                border_color.0 = apply_brightness(original.border, hover_brightness.0);
            }
            _ => {
                bg_color.0 = original.background;
                border_color.0 = original.border;
            }
        }
    }
}

/// Comprehensive button interaction system that handles all hover/pressed states
/// This system sets the target values for the animation system to interpolate
pub fn handle_button_interaction(
    mut query: Query<(
        &Interaction,
        &mut ButtonAnimationState,
        Option<&HoverScale>,
    ), (Changed<Interaction>, With<StyledButton>)>,
) {
    for (interaction, mut animation, hover_scale) in &mut query {
        match interaction {
            Interaction::Pressed => {
                // Set target for pressed state
                animation.target_scale = hover_scale.map_or(1.0, |s| s.0 * 0.98); // More subtle press
                animation.target_color_blend = 1.0; // Full pressed colors
            }
            Interaction::Hovered => {
                // Set target for hover state
                animation.target_scale = hover_scale.map_or(1.0, |s| s.0);
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
            border_color.0 = target_border;
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
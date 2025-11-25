//! Animation systems for processing UI animations.

use bevy::prelude::*;

use super::easing::Easing;
use super::transitions::Transition;
use super::types::{
    AnimationCategory, AnimationOriginals, DisableAutoAnimation, EnterAnimating, ExitAnimating,
    UiAnimation,
};
use crate::theme::UiTheme;

/// System to automatically add UiAnimation to entities with Interaction + AnimationCategory.
///
/// This runs before other animation systems and applies per-category defaults from the theme.
/// Entities with `DisableAutoAnimation` are skipped.
pub fn auto_add_animation(
    mut commands: Commands,
    theme: Res<UiTheme>,
    query: Query<
        (Entity, &AnimationCategory),
        (
            Added<Interaction>,
            Without<UiAnimation>,
            Without<DisableAutoAnimation>,
        ),
    >,
) {
    for (entity, category) in &query {
        let defaults = theme.animation.get_category_defaults(*category);
        let animation = UiAnimation::from_category_defaults(defaults);
        commands.entity(entity).insert(animation);
    }
}

/// System to handle interaction-based animation targets
pub fn update_interaction_animations(
    mut query: Query<(&Interaction, &mut UiAnimation), Changed<Interaction>>,
) {
    for (interaction, mut animation) in &mut query {
        // Clone the target to avoid borrow issues
        let target = match interaction {
            Interaction::None => animation.interaction.idle.clone(),
            Interaction::Hovered => animation.interaction.hover.clone(),
            Interaction::Pressed => animation.interaction.pressed.clone(),
        };

        // Set animation targets from interaction state
        animation.state.target_scale = target.scale.unwrap_or(1.0);
        animation.state.target_brightness = target.brightness.unwrap_or(1.0);
        animation.state.target_opacity = target.opacity.unwrap_or(1.0);
        animation.state.target_color_blend = if target.color_target.is_some() {
            1.0
        } else {
            0.0
        };
        animation.state.target_translation = target.translation.unwrap_or(bevy::prelude::Vec2::ZERO);
        animation.state.is_animating = true;
    }
}

/// System to tick animation states towards their targets
pub fn tick_animation_states(mut query: Query<&mut UiAnimation>, time: Res<Time>) {
    let delta = time.delta_secs();

    for mut animation in &mut query {
        if !animation.state.is_animating {
            continue;
        }

        let speed = animation.state.animation_speed * delta;
        let state = &mut animation.state;

        // Animate scale
        let scale_diff = state.target_scale - state.current_scale;
        if scale_diff.abs() > 0.001 {
            state.current_scale += scale_diff * speed;
        } else {
            state.current_scale = state.target_scale;
        }

        // Animate brightness
        let brightness_diff = state.target_brightness - state.current_brightness;
        if brightness_diff.abs() > 0.001 {
            state.current_brightness += brightness_diff * speed;
        } else {
            state.current_brightness = state.target_brightness;
        }

        // Animate opacity
        let opacity_diff = state.target_opacity - state.current_opacity;
        if opacity_diff.abs() > 0.001 {
            state.current_opacity += opacity_diff * speed;
        } else {
            state.current_opacity = state.target_opacity;
        }

        // Animate color blend
        let blend_diff = state.target_color_blend - state.current_color_blend;
        if blend_diff.abs() > 0.001 {
            state.current_color_blend += blend_diff * speed;
        } else {
            state.current_color_blend = state.target_color_blend;
        }

        // Animate translation
        let translation_diff = state.target_translation - state.current_translation;
        if translation_diff.length() > 0.1 {
            state.current_translation += translation_diff * speed;
        } else {
            state.current_translation = state.target_translation;
        }

        // Check if animation is complete
        if state.is_complete() {
            state.is_animating = false;
        }
    }
}

/// System to apply animation state to transforms
pub fn apply_animation_transforms(
    mut query: Query<(&UiAnimation, &mut Transform, Option<&AnimationOriginals>)>,
) {
    for (animation, mut transform, originals) in &mut query {
        let state = &animation.state;

        // Get base scale from originals or use Vec3::ONE
        let base_scale = originals
            .map(|o| o.scale)
            .unwrap_or(Vec3::ONE);

        // Apply scale
        transform.scale = base_scale * state.current_scale;

        // Get base translation from originals or use current
        let base_translation = originals
            .map(|o| o.translation)
            .unwrap_or(transform.translation);

        // Apply translation offset (convert Vec2 to Vec3)
        transform.translation = base_translation
            + Vec3::new(
                state.current_translation.x,
                state.current_translation.y,
                0.0,
            );
    }
}

/// System to apply animation state to colors
pub fn apply_animation_colors(
    mut query: Query<(
        &UiAnimation,
        &mut BackgroundColor,
        Option<&mut BorderColor>,
        Option<&AnimationOriginals>,
    )>,
) {
    for (animation, mut bg_color, border_color, originals) in &mut query {
        let state = &animation.state;

        // Apply brightness to background
        if let Some(original_bg) = originals.and_then(|o| o.background) {
            bg_color.0 = apply_brightness(original_bg, state.current_brightness);
        }

        // Apply brightness to border if present
        if let (Some(mut border), Some(original_border)) =
            (border_color, originals.and_then(|o| o.border))
        {
            *border = BorderColor::all(apply_brightness(original_border, state.current_brightness));
        }
    }
}

/// System to initialize animation originals when UiAnimation is added
pub fn init_animation_originals(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &Transform,
            Option<&BackgroundColor>,
            Option<&BorderColor>,
        ),
        Added<UiAnimation>,
    >,
) {
    for (entity, transform, bg_color, border_color) in &query {
        commands.entity(entity).insert(AnimationOriginals {
            background: bg_color.map(|c| c.0),
            border: border_color.map(|c| c.top), // Use .top as representative color
            scale: transform.scale,
            translation: transform.translation,
        });
    }
}

/// System to process enter animations
pub fn process_enter_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UiAnimation, &mut Transform), With<EnterAnimating>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut animation, mut transform) in &mut query {
        if let Some(ref mut mount_anim) = animation.enter {
            if !mount_anim.playing {
                mount_anim.play();
            }

            let duration = mount_anim.transition.duration();
            if duration > 0.0 {
                mount_anim.progress += delta / duration;
            } else {
                mount_anim.progress = 1.0;
            }

            // Apply transition effects
            apply_mount_transition(&mount_anim.transition, mount_anim.progress, &mut transform);

            if mount_anim.is_complete() {
                mount_anim.playing = false;
                commands.entity(entity).remove::<EnterAnimating>();
            }
        } else {
            commands.entity(entity).remove::<EnterAnimating>();
        }
    }
}

/// System to process exit animations
pub fn process_exit_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UiAnimation, &mut Transform), With<ExitAnimating>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (entity, mut animation, mut transform) in &mut query {
        if let Some(ref mut mount_anim) = animation.exit {
            if !mount_anim.playing {
                mount_anim.play_reverse();
            }

            let duration = mount_anim.transition.duration();
            if duration > 0.0 {
                mount_anim.progress -= delta / duration;
            } else {
                mount_anim.progress = 0.0;
            }

            // Apply transition effects (in reverse)
            apply_mount_transition(&mount_anim.transition, mount_anim.progress, &mut transform);

            if mount_anim.is_complete() {
                // Despawn entity after exit animation completes
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).remove::<ExitAnimating>();
        }
    }
}

/// Apply mount transition effects to a transform
fn apply_mount_transition(transition: &Transition, progress: f32, transform: &mut Transform) {
    let easing = transition.easing().unwrap_or(Easing::EaseOut);
    let t = easing.apply(progress);

    match transition {
        Transition::FadeIn { .. } | Transition::FadeOut { .. } => {
            // Opacity is handled separately
        }
        Transition::SlideIn {
            direction,
            distance,
            ..
        } => {
            let offset = direction.as_vec2(*distance) * (1.0 - t);
            transform.translation.x += offset.x;
            transform.translation.y += offset.y;
        }
        Transition::SlideOut {
            direction,
            distance,
            ..
        } => {
            let offset = direction.as_vec2(*distance) * t;
            transform.translation.x += offset.x;
            transform.translation.y += offset.y;
        }
        Transition::ScaleIn { from, .. } => {
            let scale = *from + (1.0 - *from) * t;
            transform.scale = Vec3::splat(scale);
        }
        Transition::ScaleOut { to, .. } => {
            let scale = 1.0 + (*to - 1.0) * t;
            transform.scale = Vec3::splat(scale);
        }
        Transition::FadeSlide {
            direction,
            distance,
            ..
        } => {
            let offset = direction.as_vec2(*distance) * (1.0 - t);
            transform.translation.x += offset.x;
            transform.translation.y += offset.y;
        }
        Transition::FadeScale { scale, .. } => {
            let s = *scale + (1.0 - *scale) * t;
            transform.scale = Vec3::splat(s);
        }
        Transition::Bounce { intensity, .. } => {
            // Bounce effect using elastic easing
            let bounce_t = Easing::ElasticOut.apply(progress);
            transform.scale = Vec3::splat(1.0 + *intensity * (bounce_t - 1.0).abs());
        }
        Transition::Shake { intensity, .. } => {
            // Shake effect
            let shake_amount = *intensity * (1.0 - t);
            let phase = progress * 20.0 * std::f32::consts::PI;
            transform.translation.x += shake_amount * phase.sin();
        }
        Transition::Pulse { scale, .. } => {
            let pulse_t = (progress * std::f32::consts::PI).sin();
            transform.scale = Vec3::splat(1.0 + (*scale - 1.0) * pulse_t);
        }
        Transition::None => {}
    }
}

/// Helper function to apply brightness to a color
fn apply_brightness(color: Color, brightness: f32) -> Color {
    let rgba = color.to_linear();
    Color::LinearRgba(LinearRgba {
        red: (rgba.red * brightness).min(1.0),
        green: (rgba.green * brightness).min(1.0),
        blue: (rgba.blue * brightness).min(1.0),
        alpha: rgba.alpha,
    })
}

/// Helper function to lerp between colors
pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    let from_linear = from.to_linear();
    let to_linear = to.to_linear();

    Color::LinearRgba(LinearRgba {
        red: from_linear.red + (to_linear.red - from_linear.red) * t,
        green: from_linear.green + (to_linear.green - from_linear.green) * t,
        blue: from_linear.blue + (to_linear.blue - from_linear.blue) * t,
        alpha: from_linear.alpha + (to_linear.alpha - from_linear.alpha) * t,
    })
}

//! Button component types and markers

use bevy::prelude::*;

/// Marker component for styled buttons
#[derive(Component)]
pub struct StyledButton;

/// Component storing button-specific state colors
/// This is kept for compatibility with existing button code
/// but new code should use HoverColors from systems::hover
#[derive(Component)]
pub struct ButtonStateColors {
    pub normal_bg: Color,
    pub hover_bg: Color,
    pub pressed_bg: Color,
    pub normal_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

/// Component for button-specific animation state
/// This is kept for compatibility with existing button code
/// but new code should use HoverAnimationState from systems::hover
#[derive(Component)]
pub struct ButtonAnimationState {
    /// Current scale value (animated)
    pub current_scale: f32,
    /// Target scale value
    pub target_scale: f32,
    /// Current color blend factor (0.0 = normal, 1.0 = hover/pressed)
    pub current_color_blend: f32,
    /// Target color blend factor
    pub target_color_blend: f32,
    /// Animation speed (higher = faster)
    pub animation_speed: f32,
}

// Note: HoverScale, HoverBrightness, and OriginalColors have been moved to
// systems::hover for universal use across all UI elements
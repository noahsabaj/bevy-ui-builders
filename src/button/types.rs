//! Button component types and markers

use bevy::prelude::*;
use crate::styles::ButtonStyle;

/// Marker component for styled buttons
#[derive(Component)]
pub struct StyledButton;

/// Component storing the button's style for hover effects
#[derive(Component, Copy, Clone)]
pub struct ButtonStyleComponent(pub ButtonStyle);

/// Component storing button state colors
#[derive(Component)]
pub struct ButtonStateColors {
    pub normal_bg: Color,
    pub hover_bg: Color,
    pub pressed_bg: Color,
    pub normal_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

/// Component storing hover scale factor
#[derive(Component)]
pub struct HoverScale(pub f32);

/// Component storing hover brightness factor
#[derive(Component)]
pub struct HoverBrightness(pub f32);

/// Component for smooth animation of button properties
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

/// Component storing original colors for hover effects
#[derive(Component)]
pub struct OriginalColors {
    pub background: Color,
    pub border: Color,
}
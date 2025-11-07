//! Universal hover effects system for all UI elements
//!
//! Provides centralized hover components and systems that can be used by any UI element
//! to create consistent hover effects across the entire interface.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Component for hover scale effects - scales the element on hover
#[derive(Component, Clone)]
pub struct HoverScale(pub f32);

impl Default for HoverScale {
    fn default() -> Self {
        Self(1.05) // 5% scale increase by default
    }
}

/// Component for hover brightness effects - brightens colors on hover
#[derive(Component, Clone)]
pub struct HoverBrightness(pub f32);

impl Default for HoverBrightness {
    fn default() -> Self {
        Self(1.2) // 20% brighter by default
    }
}

/// Component for hover opacity effects
#[derive(Component, Clone)]
pub struct HoverOpacity {
    pub normal: f32,
    pub hovered: f32,
}

impl Default for HoverOpacity {
    fn default() -> Self {
        Self {
            normal: 1.0,
            hovered: 0.9,
        }
    }
}

/// Component for hover outline effects
#[derive(Component, Clone)]
pub struct HoverOutline {
    pub color: Color,
    pub width: Val,
    pub active: bool,
}

impl Default for HoverOutline {
    fn default() -> Self {
        Self {
            color: Color::srgba(1.0, 1.0, 1.0, 0.3),
            width: Val::Px(2.0),
            active: false,
        }
    }
}

/// Component for hover color changes
#[derive(Component, Clone)]
pub struct HoverColors {
    pub normal_bg: Color,
    pub hover_bg: Color,
    pub normal_border: Color,
    pub hover_border: Color,
}

/// Component storing original colors for restoration
#[derive(Component)]
pub struct OriginalColors {
    pub background: Color,
    pub border: Color,
}

/// Component for smooth hover animations
#[derive(Component)]
pub struct HoverAnimationState {
    /// Current scale value (animated)
    pub current_scale: f32,
    /// Target scale value
    pub target_scale: f32,
    /// Current opacity value (animated)
    pub current_opacity: f32,
    /// Target opacity value
    pub target_opacity: f32,
    /// Current color blend factor (0.0 = normal, 1.0 = hover)
    pub current_blend: f32,
    /// Target color blend factor
    pub target_blend: f32,
    /// Animation speed (higher = faster)
    pub animation_speed: f32,
}

impl Default for HoverAnimationState {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            current_opacity: 1.0,
            target_opacity: 1.0,
            current_blend: 0.0,
            target_blend: 0.0,
            animation_speed: 10.0,
        }
    }
}

// Note: HoverCursor functionality removed due to Bevy 0.16 cursor API changes
// Cursor icons now require window entity component manipulation

/// Resource for global hover configuration
#[derive(Resource)]
pub struct HoverConfig {
    pub default_scale: f32,
    pub default_brightness: f32,
    pub animation_speed: f32,
    pub enable_sounds: bool,
}

impl Default for HoverConfig {
    fn default() -> Self {
        Self {
            default_scale: 1.05,
            default_brightness: 1.2,
            animation_speed: 10.0,
            enable_sounds: false,
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Universal system to handle hover scale effects
pub fn universal_hover_scale(
    mut query: Query<(
        &Interaction,
        &HoverScale,
        &mut HoverAnimationState,
    ), Changed<Interaction>>,
) {
    for (interaction, hover_scale, mut animation) in &mut query {
        match interaction {
            Interaction::Hovered => {
                animation.target_scale = hover_scale.0;
            }
            Interaction::Pressed => {
                animation.target_scale = hover_scale.0 * 0.98; // Slightly smaller when pressed
            }
            Interaction::None => {
                animation.target_scale = 1.0;
            }
        }
    }
}

/// Universal system to handle hover color effects
pub fn universal_hover_colors(
    mut query: Query<(
        &Interaction,
        &HoverColors,
        &mut HoverAnimationState,
    ), Changed<Interaction>>,
) {
    for (interaction, _colors, mut animation) in &mut query {
        match interaction {
            Interaction::Hovered => {
                animation.target_blend = 0.5; // Halfway to hover colors
            }
            Interaction::Pressed => {
                animation.target_blend = 1.0; // Full hover colors
            }
            Interaction::None => {
                animation.target_blend = 0.0; // Normal colors
            }
        }
    }
}

/// Universal system to handle hover opacity effects
pub fn universal_hover_opacity(
    mut query: Query<(
        &Interaction,
        &HoverOpacity,
        &mut HoverAnimationState,
    ), Changed<Interaction>>,
) {
    for (interaction, hover_opacity, mut animation) in &mut query {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                animation.target_opacity = hover_opacity.hovered;
            }
            Interaction::None => {
                animation.target_opacity = hover_opacity.normal;
            }
        }
    }
}

/// Universal system to handle hover outline effects
pub fn universal_hover_outline(
    mut query: Query<(
        Entity,
        &Interaction,
        &HoverOutline,
        &mut BorderColor,
        Option<&OriginalColors>,
    ), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction, outline, mut border_color, original) in &mut query {

        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                if !outline.active {
                    // Store original border if not already stored
                    if original.is_none() {
                        commands.entity(entity).insert(OriginalColors {
                            background: Color::NONE,
                            border: border_color.top,  // Store one side as representative
                        });
                    }
                    *border_color = BorderColor::all(outline.color);
                    // Note: Border width would need to be set on Node component at spawn time
                }
            }
            Interaction::None => {
                if let Some(orig) = original {
                    *border_color = BorderColor::all(orig.border);
                }
            }
        }
    }
}

/// System to smoothly animate hover transitions
pub fn animate_hover_transitions(
    mut query: Query<(
        &mut HoverAnimationState,
        Option<&mut Transform>,
        Option<(&HoverColors, &mut BackgroundColor, &mut BorderColor)>,
    )>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut animation, transform, colors) in &mut query {
        // Animate scale
        if let Some(mut transform) = transform {
            let scale_diff = animation.target_scale - animation.current_scale;
            if scale_diff.abs() > 0.001 {
                animation.current_scale += scale_diff * animation.animation_speed * delta;
                transform.scale = Vec3::splat(animation.current_scale);
            }
        }

        // Animate colors
        if let Some((hover_colors, mut bg_color, mut border_color)) = colors {
            let blend_diff = animation.target_blend - animation.current_blend;
            if blend_diff.abs() > 0.001 {
                animation.current_blend += blend_diff * animation.animation_speed * delta;

                // Blend between normal and hover colors
                bg_color.0 = lerp_color(
                    hover_colors.normal_bg,
                    hover_colors.hover_bg,
                    animation.current_blend,
                );
                *border_color = BorderColor::all(lerp_color(
                    hover_colors.normal_border,
                    hover_colors.hover_border,
                    animation.current_blend,
                ));
            }
        }

        // Animate opacity
        let opacity_diff = animation.target_opacity - animation.current_opacity;
        if opacity_diff.abs() > 0.001 {
            animation.current_opacity += opacity_diff * animation.animation_speed * delta;
            // Note: Actual opacity application would need to be done per-element type
        }
    }
}

// Note: Cursor icon hover system removed due to Bevy 0.16 API changes
// Implementing cursor changes requires window entity component manipulation

/// Initialize hover animation state when hover components are added
pub fn init_hover_animation(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            Added<HoverScale>,
            Added<HoverColors>,
            Added<HoverOpacity>,
        )>,
    >,
) {
    for entity in &query {
        commands.entity(entity).try_insert(HoverAnimationState::default());
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

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

/// Apply brightness multiplier to a color
pub fn apply_brightness(color: Color, brightness: f32) -> Color {
    let rgba = color.to_linear();
    Color::LinearRgba(LinearRgba {
        red: (rgba.red * brightness).min(1.0),
        green: (rgba.green * brightness).min(1.0),
        blue: (rgba.blue * brightness).min(1.0),
        alpha: rgba.alpha,
    })
}

// ============================================================================
// PLUGIN
// ============================================================================

// Plugin that adds universal hover effect systems for all UI elements
define_plugin!(HoverPlugin {
    custom_init: |app: &mut App| {
        app.insert_resource(HoverConfig::default());
    },
    update: [
        // Initialize animation states
        init_hover_animation,

        // Handle hover state changes
        universal_hover_scale,
        universal_hover_colors,
        universal_hover_opacity,
        universal_hover_outline,

        // Animate transitions
        animate_hover_transitions,
    ]
});
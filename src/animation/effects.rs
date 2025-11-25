//! Hover effect types for UI animations.

use bevy::prelude::*;

/// Effects that can be applied on hover/interaction.
///
/// Multiple effects can be combined for rich feedback.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("Click Me")
///     .on_hover(HoverEffect::Scale(1.05))
///     .on_hover(HoverEffect::Brightness(1.1))
///     .build(parent);
/// ```
#[derive(Clone, Debug)]
pub enum HoverEffect {
    /// Scale the element (1.0 = normal size)
    Scale(f32),

    /// Adjust brightness (1.0 = normal, >1 = brighter)
    Brightness(f32),

    /// Shift color towards a target
    ColorShift {
        /// Target color to blend towards
        to: Color,
    },

    /// Add a glow effect to the border
    BorderGlow {
        /// Glow color
        color: Color,
        /// Glow width/intensity
        width: f32,
    },

    /// Lift the element up (creates a floating effect)
    Lift {
        /// Distance to lift (in pixels)
        distance: f32,
    },

    /// Change opacity (0.0 = transparent, 1.0 = opaque)
    Opacity(f32),
}

impl HoverEffect {
    /// Create a subtle scale effect
    pub fn subtle_scale() -> Self {
        Self::Scale(1.02)
    }

    /// Create a pronounced scale effect
    pub fn pop_scale() -> Self {
        Self::Scale(1.05)
    }

    /// Create a subtle brightness effect
    pub fn subtle_brightness() -> Self {
        Self::Brightness(1.05)
    }

    /// Create a pronounced brightness effect
    pub fn glow_brightness() -> Self {
        Self::Brightness(1.2)
    }

    /// Create a lift effect
    pub fn lift(distance: f32) -> Self {
        Self::Lift { distance }
    }

    /// Create a border glow effect
    pub fn border_glow(color: Color) -> Self {
        Self::BorderGlow { color, width: 2.0 }
    }
}

// Allow converting f32 to Scale effect for ergonomic API
impl From<f32> for HoverEffect {
    fn from(scale: f32) -> Self {
        Self::Scale(scale)
    }
}

/// Press effects (applied when element is pressed/active)
#[derive(Clone, Debug)]
pub enum PressEffect {
    /// Scale down slightly (gives "pushed" feeling)
    ScaleDown(f32),

    /// Darken slightly
    Darken(f32),

    /// Move down slightly
    PushDown(f32),
}

impl Default for PressEffect {
    fn default() -> Self {
        Self::ScaleDown(0.98)
    }
}

/// Collection of hover effects that can be applied together
#[derive(Clone, Debug, Default)]
pub struct HoverEffects {
    effects: Vec<HoverEffect>,
}

impl HoverEffects {
    /// Create empty effects collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an effect
    pub fn add(mut self, effect: HoverEffect) -> Self {
        self.effects.push(effect);
        self
    }

    /// Add scale effect
    pub fn scale(self, scale: f32) -> Self {
        self.add(HoverEffect::Scale(scale))
    }

    /// Add brightness effect
    pub fn brightness(self, brightness: f32) -> Self {
        self.add(HoverEffect::Brightness(brightness))
    }

    /// Add lift effect
    pub fn lift(self, distance: f32) -> Self {
        self.add(HoverEffect::Lift { distance })
    }

    /// Get all effects
    pub fn effects(&self) -> &[HoverEffect] {
        &self.effects
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}

impl FromIterator<HoverEffect> for HoverEffects {
    fn from_iter<T: IntoIterator<Item = HoverEffect>>(iter: T) -> Self {
        Self {
            effects: iter.into_iter().collect(),
        }
    }
}

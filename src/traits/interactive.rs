//! Interactive builder traits.

use bevy::prelude::*;

use super::UiBuilder;
use crate::animation::{AnimationPreset, HoverEffect, Transition, UiAnimation};

/// Trait for builders that create interactive elements.
///
/// This provides consistent APIs for disabled states, hover effects,
/// and animations across all interactive components.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("Click")
///     .disabled()
///     .on_hover(HoverEffect::Scale(1.05))
///     .animation(AnimationPreset::Punchy)
///     .build(parent);
/// ```
pub trait InteractiveBuilder: UiBuilder {
    /// Mark this element as disabled (non-interactive)
    fn disabled(self) -> Self;

    /// Set disabled state explicitly
    fn set_disabled(self, disabled: bool) -> Self;

    /// Add a hover effect
    fn on_hover(self, effect: impl Into<HoverEffect>) -> Self;

    /// Add a press/active effect
    fn on_press(self, effect: impl Into<HoverEffect>) -> Self;

    /// Apply an animation preset
    fn animation(self, preset: AnimationPreset) -> Self;

    /// Set custom animation configuration
    fn animation_config(self, animation: UiAnimation) -> Self;

    /// Add an enter (mount) animation
    fn enter_animation(self, transition: Transition) -> Self;

    /// Add an exit (unmount) animation
    fn exit_animation(self, transition: Transition) -> Self;

    /// Disable automatic animation for this element.
    ///
    /// By default, interactive elements automatically receive animation
    /// based on their category (button, slider, etc.). Call this to
    /// prevent auto-animation.
    fn no_animation(self) -> Self;
}

/// Configuration for interactive behavior.
#[derive(Debug, Clone, Default)]
pub struct InteractiveConfig {
    /// Whether the element is disabled
    pub disabled: bool,
    /// Hover effects to apply
    pub hover_effects: Vec<HoverEffect>,
    /// Press effects to apply
    pub press_effects: Vec<HoverEffect>,
    /// Animation preset
    pub animation_preset: Option<AnimationPreset>,
    /// Custom animation configuration
    pub custom_animation: Option<UiAnimation>,
    /// Enter transition
    pub enter_transition: Option<Transition>,
    /// Exit transition
    pub exit_transition: Option<Transition>,
    /// Disable automatic animation (inserts DisableAutoAnimation component)
    pub disable_animation: bool,
}

impl InteractiveConfig {
    /// Create a new interactive config
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any hover effects are configured
    pub fn has_hover_effects(&self) -> bool {
        !self.hover_effects.is_empty()
            || self.animation_preset.is_some()
            || self.custom_animation.is_some()
    }

    /// Check if animation should be disabled (returns true if `no_animation()` was called)
    pub fn should_disable_animation(&self) -> bool {
        self.disable_animation
    }

    /// Build a UiAnimation from this config (returns None if animation is disabled)
    pub fn build_animation(&self) -> Option<UiAnimation> {
        // If animation is disabled, don't build one (auto-add system will be blocked)
        if self.disable_animation {
            return None;
        }

        if !self.has_hover_effects() && self.enter_transition.is_none() && self.exit_transition.is_none() {
            return None;
        }

        let mut anim = if let Some(ref custom) = self.custom_animation {
            custom.clone()
        } else if let Some(preset) = self.animation_preset {
            UiAnimation::new().with_preset(preset)
        } else if !self.hover_effects.is_empty() {
            UiAnimation::from_effects(&self.hover_effects)
        } else {
            UiAnimation::new()
        };

        // Apply transitions
        if let Some(ref enter) = self.enter_transition {
            anim = anim.with_enter(enter.clone());
        }
        if let Some(ref exit) = self.exit_transition {
            anim = anim.with_exit(exit.clone());
        }

        Some(anim)
    }

    /// Add a hover effect
    pub fn add_hover_effect(&mut self, effect: HoverEffect) {
        self.hover_effects.push(effect);
    }

    /// Add a press effect
    pub fn add_press_effect(&mut self, effect: HoverEffect) {
        self.press_effects.push(effect);
    }
}

/// Marker component for disabled UI elements.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Disabled;

/// Component storing the enabled/disabled state.
#[derive(Component, Debug, Clone, Copy)]
pub struct InteractiveState {
    /// Whether the element is currently disabled
    pub disabled: bool,
    /// Whether the element is currently focused
    pub focused: bool,
    /// Whether the element is currently pressed
    pub pressed: bool,
}

impl Default for InteractiveState {
    fn default() -> Self {
        Self {
            disabled: false,
            focused: false,
            pressed: false,
        }
    }
}

impl InteractiveState {
    /// Create a new interactive state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a disabled state
    pub fn disabled() -> Self {
        Self {
            disabled: true,
            ..Self::default()
        }
    }
}

/// Macro to implement InteractiveBuilder for a builder type.
///
/// The builder must have an `interactive: InteractiveConfig` field.
///
/// # Example
///
/// ```ignore
/// pub struct MyBuilder {
///     interactive: InteractiveConfig,
///     // ... other fields
/// }
///
/// impl_interactive_builder!(MyBuilder);
/// ```
#[macro_export]
macro_rules! impl_interactive_builder {
    ($builder:ty) => {
        impl $crate::traits::InteractiveBuilder for $builder {
            fn disabled(mut self) -> Self {
                self.interactive.disabled = true;
                self
            }

            fn set_disabled(mut self, disabled: bool) -> Self {
                self.interactive.disabled = disabled;
                self
            }

            fn on_hover(mut self, effect: impl Into<$crate::animation::HoverEffect>) -> Self {
                self.interactive.hover_effects.push(effect.into());
                self
            }

            fn on_press(mut self, effect: impl Into<$crate::animation::HoverEffect>) -> Self {
                self.interactive.press_effects.push(effect.into());
                self
            }

            fn animation(mut self, preset: $crate::animation::AnimationPreset) -> Self {
                self.interactive.animation_preset = Some(preset);
                self
            }

            fn animation_config(mut self, animation: $crate::animation::UiAnimation) -> Self {
                self.interactive.custom_animation = Some(animation);
                self
            }

            fn enter_animation(mut self, transition: $crate::animation::Transition) -> Self {
                self.interactive.enter_transition = Some(transition);
                self
            }

            fn exit_animation(mut self, transition: $crate::animation::Transition) -> Self {
                self.interactive.exit_transition = Some(transition);
                self
            }

            fn no_animation(mut self) -> Self {
                self.interactive.disable_animation = true;
                self
            }
        }
    };
}

//! Unified animation system for bevy-ui-builders.
//!
//! This module provides a comprehensive animation solution for UI elements:
//!
//! - **Hover/Interaction Effects**: Scale, brightness, color shifts on hover/press
//! - **Enter/Exit Transitions**: Fade, slide, scale animations for mounting/unmounting
//! - **Easing Functions**: Full library of easing curves (ease-out, bounce, elastic, etc.)
//! - **Animation Presets**: Ready-to-use configurations (Subtle, Punchy, Playful)
//!
//! # Quick Start
//!
//! ```ignore
//! use bevy_ui_builders::animation::prelude::*;
//!
//! // Add hover effects via builder
//! ButtonBuilder::new("Click Me")
//!     .on_hover(HoverEffect::Scale(1.05))
//!     .on_hover(HoverEffect::Brightness(1.1))
//!     .build(parent);
//!
//! // Or use presets
//! ButtonBuilder::new("Punchy Button")
//!     .animation(AnimationPreset::Punchy)
//!     .build(parent);
//!
//! // Add enter/exit animations
//! DialogBuilder::new(DialogType::Info)
//!     .enter_animation(Transition::fade_scale(0.95, 0.2))
//!     .exit_animation(Transition::fade_out(0.15))
//!     .build(commands);
//! ```
//!
//! # Architecture
//!
//! The animation system uses:
//! - `UiAnimation` component for state and configuration
//! - `AnimationOriginals` to store base values for restoration
//! - `AnimationCategory` to determine per-category defaults
//! - Auto-add system that automatically applies animations to interactive elements
//! - Systems that run each frame to interpolate towards targets

mod easing;
mod effects;
mod plugin;
pub mod presets;
mod systems;
mod transitions;
mod types;

// Re-export all public types
pub use easing::Easing;
pub use effects::{HoverEffect, HoverEffects, PressEffect};
pub use plugin::AnimationPlugin;
pub use systems::lerp_color;
pub use transitions::{Direction, Transition};
pub use types::{
    AnimationCategory, AnimationOriginals, AnimationPreset, AnimationState, AnimationTarget,
    DisableAutoAnimation, EnterAnimating, ExitAnimating, InteractionAnimation, MountAnimation,
    UiAnimation,
};

/// Prelude module for convenient animation imports
pub mod prelude {
    pub use super::{
        AnimationCategory, AnimationPreset, Direction, DisableAutoAnimation, Easing, HoverEffect,
        HoverEffects, Transition, UiAnimation,
    };
}

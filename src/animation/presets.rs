//! Pre-built animation presets.

use super::types::{AnimationPreset, UiAnimation};
use super::transitions::Transition;

impl AnimationPreset {
    /// Get a description of this preset
    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "No animation effects",
            Self::Subtle => "Subtle, professional animation - scale 1.02, brightness 1.05",
            Self::Punchy => "Satisfying feedback - scale 1.05, brightness 1.15",
            Self::Playful => "Bouncy, game-like - scale 1.08, faster animations",
            Self::Lift => "Lift up effect with subtle movement",
            Self::Glow => "Brightness/glow effect",
        }
    }

    /// Create a UiAnimation from this preset
    pub fn to_animation(&self) -> UiAnimation {
        UiAnimation::new().with_preset(*self)
    }
}

/// Common transition presets for dialogs
pub mod dialog {
    use super::*;

    /// Standard dialog enter animation (fade + scale)
    pub fn enter() -> Transition {
        Transition::fade_scale(0.95, 0.2).ease_out()
    }

    /// Standard dialog exit animation
    pub fn exit() -> Transition {
        Transition::fade_scale(0.95, 0.15).ease_in()
    }

    /// Slide up dialog animation
    pub fn slide_up() -> Transition {
        Transition::fade_slide(super::super::transitions::Direction::Bottom, 30.0, 0.25).ease_out()
    }
}

/// Common transition presets for toasts/notifications
pub mod toast {
    use super::*;

    /// Toast sliding in from top
    pub fn enter_top() -> Transition {
        Transition::slide_from_top(50.0, 0.3).ease_out()
    }

    /// Toast sliding in from bottom
    pub fn enter_bottom() -> Transition {
        Transition::slide_from_bottom(50.0, 0.3).ease_out()
    }

    /// Toast exit animation
    pub fn exit() -> Transition {
        Transition::fade_out(0.2).ease_in()
    }
}

/// Common transition presets for tooltips
pub mod tooltip {
    use super::*;

    /// Tooltip fade in
    pub fn enter() -> Transition {
        Transition::fade_in(0.15).ease_out()
    }

    /// Tooltip fade out
    pub fn exit() -> Transition {
        Transition::fade_out(0.1).ease_in()
    }
}

/// Common transition presets for menus/dropdowns
pub mod menu {
    use super::*;

    /// Menu slide down animation
    pub fn enter() -> Transition {
        Transition::fade_slide(super::super::transitions::Direction::Top, 10.0, 0.15).ease_out()
    }

    /// Menu close animation
    pub fn exit() -> Transition {
        Transition::fade_out(0.1).ease_in()
    }
}

/// Common transition presets for panels/cards
pub mod panel {
    use super::*;

    /// Panel fade in
    pub fn enter() -> Transition {
        Transition::fade_in(0.2).ease_out()
    }

    /// Panel with staggered children (use with cascade)
    pub fn stagger_enter() -> Transition {
        Transition::fade_slide(super::super::transitions::Direction::Bottom, 20.0, 0.2).ease_out()
    }
}

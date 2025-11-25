//! Transition animations for enter/exit effects.

use bevy::prelude::*;

use super::easing::Easing;

/// Transition animations for mounting (enter) and unmounting (exit) UI elements.
///
/// # Example
///
/// ```ignore
/// DialogBuilder::new(DialogType::Info)
///     .enter_animation(Transition::fade_in(0.3))
///     .exit_animation(Transition::fade_out(0.2))
///     .build(commands);
/// ```
#[derive(Clone, Debug)]
pub enum Transition {
    /// Fade in from transparent
    FadeIn {
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Fade out to transparent
    FadeOut {
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Slide in from a direction
    SlideIn {
        /// Direction to slide from
        direction: Direction,
        /// Distance to slide (in pixels)
        distance: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Slide out to a direction
    SlideOut {
        /// Direction to slide to
        direction: Direction,
        /// Distance to slide (in pixels)
        distance: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Scale in from small
    ScaleIn {
        /// Starting scale (0.0 = invisible, 1.0 = normal)
        from: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Scale out to small
    ScaleOut {
        /// Ending scale
        to: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Combined fade and slide
    FadeSlide {
        /// Direction to slide from/to
        direction: Direction,
        /// Distance to slide
        distance: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Combined fade and scale
    FadeScale {
        /// Starting/ending scale
        scale: f32,
        /// Duration in seconds
        duration: f32,
        /// Easing curve
        easing: Easing,
    },

    /// Spring/bounce effect
    Bounce {
        /// Bounce intensity (0.0 to 1.0)
        intensity: f32,
        /// Duration in seconds
        duration: f32,
    },

    /// Shake effect (for errors, attention)
    Shake {
        /// Shake intensity in pixels
        intensity: f32,
        /// Duration in seconds
        duration: f32,
    },

    /// Pulse effect (scale in and out)
    Pulse {
        /// Scale amount
        scale: f32,
        /// Duration per pulse
        duration: f32,
    },

    /// No transition (instant)
    None,
}

impl Default for Transition {
    fn default() -> Self {
        Self::None
    }
}

impl Transition {
    // ========== Constructors ==========

    /// Create a fade in transition
    pub fn fade_in(duration: f32) -> Self {
        Self::FadeIn {
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a fade out transition
    pub fn fade_out(duration: f32) -> Self {
        Self::FadeOut {
            duration,
            easing: Easing::EaseIn,
        }
    }

    /// Create a slide in from top
    pub fn slide_from_top(distance: f32, duration: f32) -> Self {
        Self::SlideIn {
            direction: Direction::Top,
            distance,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a slide in from bottom
    pub fn slide_from_bottom(distance: f32, duration: f32) -> Self {
        Self::SlideIn {
            direction: Direction::Bottom,
            distance,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a slide in from left
    pub fn slide_from_left(distance: f32, duration: f32) -> Self {
        Self::SlideIn {
            direction: Direction::Left,
            distance,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a slide in from right
    pub fn slide_from_right(distance: f32, duration: f32) -> Self {
        Self::SlideIn {
            direction: Direction::Right,
            distance,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a scale in transition
    pub fn scale_in(from: f32, duration: f32) -> Self {
        Self::ScaleIn {
            from,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a scale out transition
    pub fn scale_out(to: f32, duration: f32) -> Self {
        Self::ScaleOut {
            to,
            duration,
            easing: Easing::EaseIn,
        }
    }

    /// Create a combined fade and slide transition
    pub fn fade_slide(direction: Direction, distance: f32, duration: f32) -> Self {
        Self::FadeSlide {
            direction,
            distance,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a combined fade and scale transition
    pub fn fade_scale(scale: f32, duration: f32) -> Self {
        Self::FadeScale {
            scale,
            duration,
            easing: Easing::EaseOut,
        }
    }

    /// Create a bounce transition
    pub fn bounce(intensity: f32, duration: f32) -> Self {
        Self::Bounce { intensity, duration }
    }

    /// Create a shake transition
    pub fn shake(intensity: f32, duration: f32) -> Self {
        Self::Shake { intensity, duration }
    }

    /// Create a pulse transition
    pub fn pulse(scale: f32, duration: f32) -> Self {
        Self::Pulse { scale, duration }
    }

    // ========== Modifiers ==========

    /// Set the easing curve for this transition
    pub fn ease(self, easing: Easing) -> Self {
        match self {
            Self::FadeIn { duration, .. } => Self::FadeIn { duration, easing },
            Self::FadeOut { duration, .. } => Self::FadeOut { duration, easing },
            Self::SlideIn {
                direction,
                distance,
                duration,
                ..
            } => Self::SlideIn {
                direction,
                distance,
                duration,
                easing,
            },
            Self::SlideOut {
                direction,
                distance,
                duration,
                ..
            } => Self::SlideOut {
                direction,
                distance,
                duration,
                easing,
            },
            Self::ScaleIn { from, duration, .. } => Self::ScaleIn {
                from,
                duration,
                easing,
            },
            Self::ScaleOut { to, duration, .. } => Self::ScaleOut {
                to,
                duration,
                easing,
            },
            Self::FadeSlide {
                direction,
                distance,
                duration,
                ..
            } => Self::FadeSlide {
                direction,
                distance,
                duration,
                easing,
            },
            Self::FadeScale { scale, duration, .. } => Self::FadeScale {
                scale,
                duration,
                easing,
            },
            other => other, // Bounce, Shake, Pulse, None don't have easing
        }
    }

    /// Apply ease-out easing
    pub fn ease_out(self) -> Self {
        self.ease(Easing::EaseOut)
    }

    /// Apply ease-in easing
    pub fn ease_in(self) -> Self {
        self.ease(Easing::EaseIn)
    }

    /// Apply ease-in-out easing
    pub fn ease_in_out(self) -> Self {
        self.ease(Easing::EaseInOut)
    }

    /// Apply spring easing
    pub fn spring(self) -> Self {
        self.ease(Easing::Spring)
    }

    // ========== Properties ==========

    /// Get the duration of this transition
    pub fn duration(&self) -> f32 {
        match self {
            Self::FadeIn { duration, .. } => *duration,
            Self::FadeOut { duration, .. } => *duration,
            Self::SlideIn { duration, .. } => *duration,
            Self::SlideOut { duration, .. } => *duration,
            Self::ScaleIn { duration, .. } => *duration,
            Self::ScaleOut { duration, .. } => *duration,
            Self::FadeSlide { duration, .. } => *duration,
            Self::FadeScale { duration, .. } => *duration,
            Self::Bounce { duration, .. } => *duration,
            Self::Shake { duration, .. } => *duration,
            Self::Pulse { duration, .. } => *duration,
            Self::None => 0.0,
        }
    }

    /// Get the easing curve if applicable
    pub fn easing(&self) -> Option<Easing> {
        match self {
            Self::FadeIn { easing, .. } => Some(*easing),
            Self::FadeOut { easing, .. } => Some(*easing),
            Self::SlideIn { easing, .. } => Some(*easing),
            Self::SlideOut { easing, .. } => Some(*easing),
            Self::ScaleIn { easing, .. } => Some(*easing),
            Self::ScaleOut { easing, .. } => Some(*easing),
            Self::FadeSlide { easing, .. } => Some(*easing),
            Self::FadeScale { easing, .. } => Some(*easing),
            _ => None,
        }
    }
}

/// Direction for slide transitions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
    /// From/to top
    #[default]
    Top,
    /// From/to bottom
    Bottom,
    /// From/to left
    Left,
    /// From/to right
    Right,
}

impl Direction {
    /// Get the translation vector for this direction
    pub fn as_vec2(&self, distance: f32) -> Vec2 {
        match self {
            Direction::Top => Vec2::new(0.0, -distance),
            Direction::Bottom => Vec2::new(0.0, distance),
            Direction::Left => Vec2::new(-distance, 0.0),
            Direction::Right => Vec2::new(distance, 0.0),
        }
    }

    /// Get the opposite direction
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

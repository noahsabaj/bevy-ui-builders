//! Easing functions for smooth animations.

use std::f32::consts::PI;

/// Easing curve types for animations.
///
/// These control how animations progress over time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Easing {
    /// Linear interpolation (constant speed)
    Linear,

    /// Ease in (slow start, fast end)
    EaseIn,

    /// Ease out (fast start, slow end)
    #[default]
    EaseOut,

    /// Ease in-out (slow start and end)
    EaseInOut,

    /// Quadratic ease in
    QuadIn,

    /// Quadratic ease out
    QuadOut,

    /// Quadratic ease in-out
    QuadInOut,

    /// Cubic ease in
    CubicIn,

    /// Cubic ease out
    CubicOut,

    /// Cubic ease in-out
    CubicInOut,

    /// Quartic ease in
    QuartIn,

    /// Quartic ease out
    QuartOut,

    /// Quartic ease in-out
    QuartInOut,

    /// Exponential ease in
    ExpoIn,

    /// Exponential ease out
    ExpoOut,

    /// Exponential ease in-out
    ExpoInOut,

    /// Back ease in (overshoot at start)
    BackIn,

    /// Back ease out (overshoot at end)
    BackOut,

    /// Back ease in-out (overshoot both)
    BackInOut,

    /// Elastic ease in (bounce at start)
    ElasticIn,

    /// Elastic ease out (bounce at end)
    ElasticOut,

    /// Elastic ease in-out
    ElasticInOut,

    /// Bounce ease in
    BounceIn,

    /// Bounce ease out
    BounceOut,

    /// Bounce ease in-out
    BounceInOut,

    /// Spring-like motion
    Spring,
}

impl Easing {
    /// Apply the easing function to a value t in [0, 1]
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,

            // Basic ease
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }

            // Quadratic
            Easing::QuadIn => t * t,
            Easing::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }

            // Cubic
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => 1.0 - (1.0 - t).powi(3),
            Easing::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }

            // Quartic
            Easing::QuartIn => t * t * t * t,
            Easing::QuartOut => 1.0 - (1.0 - t).powi(4),
            Easing::QuartInOut => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }

            // Exponential
            Easing::ExpoIn => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * t - 10.0)
                }
            }
            Easing::ExpoOut => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            Easing::ExpoInOut => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0_f32.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0
                }
            }

            // Back (overshoot)
            Easing::BackIn => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Easing::BackOut => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::BackInOut => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }

            // Elastic
            Easing::ElasticIn => {
                let c4 = (2.0 * PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
            Easing::ElasticOut => {
                let c4 = (2.0 * PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            Easing::ElasticInOut => {
                let c5 = (2.0 * PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
                } else {
                    (2.0_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
                }
            }

            // Bounce
            Easing::BounceIn => 1.0 - Easing::BounceOut.apply(1.0 - t),
            Easing::BounceOut => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Easing::BounceInOut => {
                if t < 0.5 {
                    (1.0 - Easing::BounceOut.apply(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Easing::BounceOut.apply(2.0 * t - 1.0)) / 2.0
                }
            }

            // Spring
            Easing::Spring => {
                let c4 = (2.0 * PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }

    /// Interpolate between two values using this easing
    pub fn lerp(&self, from: f32, to: f32, t: f32) -> f32 {
        let eased = self.apply(t);
        from + (to - from) * eased
    }

    /// Interpolate between two Vec2 values
    pub fn lerp_vec2(&self, from: bevy::prelude::Vec2, to: bevy::prelude::Vec2, t: f32) -> bevy::prelude::Vec2 {
        let eased = self.apply(t);
        from + (to - from) * eased
    }

    /// Interpolate between two Vec3 values
    pub fn lerp_vec3(&self, from: bevy::prelude::Vec3, to: bevy::prelude::Vec3, t: f32) -> bevy::prelude::Vec3 {
        let eased = self.apply(t);
        from + (to - from) * eased
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        let easings = [
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::CubicIn,
            Easing::CubicOut,
            Easing::BounceOut,
        ];

        for easing in easings {
            assert_eq!(easing.apply(0.0), 0.0, "{:?} at t=0", easing);
            assert!(
                (easing.apply(1.0) - 1.0).abs() < 0.001,
                "{:?} at t=1",
                easing
            );
        }
    }

    #[test]
    fn test_lerp() {
        let easing = Easing::Linear;
        assert_eq!(easing.lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(easing.lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(easing.lerp(0.0, 100.0, 1.0), 100.0);
    }
}

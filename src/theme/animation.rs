//! Animation settings for UI transitions.

use crate::animation::{AnimationCategory, Transition};

/// Animation settings for the theme.
#[derive(Clone, Debug)]
pub struct ThemeAnimation {
    /// Duration presets
    pub durations: AnimationDurations,
    /// Default easing curves
    pub easing: AnimationEasing,
    /// Global hover effect defaults
    pub hover: HoverDefaults,
    /// Per-category animation defaults
    pub categories: AnimationCategories,
}

impl Default for ThemeAnimation {
    fn default() -> Self {
        Self {
            durations: AnimationDurations::default(),
            easing: AnimationEasing::default(),
            hover: HoverDefaults::default(),
            categories: AnimationCategories::default(),
        }
    }
}

impl ThemeAnimation {
    /// Get the defaults for a specific animation category
    pub fn get_category_defaults(&self, category: AnimationCategory) -> &CategoryDefaults {
        match category {
            AnimationCategory::Button => &self.categories.button,
            AnimationCategory::Slider => &self.categories.slider,
            AnimationCategory::Input => &self.categories.input,
            AnimationCategory::Container => &self.categories.container,
            AnimationCategory::Overlay => &self.categories.overlay,
        }
    }
}

/// Per-category animation configurations
#[derive(Clone, Debug)]
pub struct AnimationCategories {
    /// Button animation defaults (also used for checkboxes, radio buttons)
    pub button: CategoryDefaults,
    /// Slider handle animation defaults
    pub slider: CategoryDefaults,
    /// Input field animation defaults
    pub input: CategoryDefaults,
    /// Container/panel animation defaults
    pub container: CategoryDefaults,
    /// Overlay/dialog animation defaults
    pub overlay: CategoryDefaults,
}

impl Default for AnimationCategories {
    fn default() -> Self {
        Self {
            button: CategoryDefaults {
                hover_scale: 1.08,
                hover_brightness: 1.3,
                press_scale: 0.94,
                animation_speed: 12.0,
                enter: None,
                exit: None,
            },
            slider: CategoryDefaults {
                hover_scale: 1.15,
                hover_brightness: 1.0,
                press_scale: 1.0,
                animation_speed: 10.0,
                enter: None,
                exit: None,
            },
            input: CategoryDefaults {
                hover_scale: 1.0,
                hover_brightness: 1.0,
                press_scale: 1.0,
                animation_speed: 10.0,
                enter: None,
                exit: None,
            },
            container: CategoryDefaults {
                hover_scale: 1.0,
                hover_brightness: 1.0,
                press_scale: 1.0,
                animation_speed: 10.0,
                enter: Some(Transition::fade_in(0.15)),
                exit: None,
            },
            overlay: CategoryDefaults {
                hover_scale: 1.0,
                hover_brightness: 1.0,
                press_scale: 1.0,
                animation_speed: 10.0,
                enter: Some(Transition::fade_scale(0.95, 0.2)),
                exit: Some(Transition::fade_out(0.15)),
            },
        }
    }
}

/// Animation defaults for a specific component category
#[derive(Clone, Debug)]
pub struct CategoryDefaults {
    /// Scale factor on hover (1.0 = no change)
    pub hover_scale: f32,
    /// Brightness multiplier on hover (1.0 = no change)
    pub hover_brightness: f32,
    /// Scale factor when pressed (1.0 = no change)
    pub press_scale: f32,
    /// Animation interpolation speed
    pub animation_speed: f32,
    /// Enter/mount transition
    pub enter: Option<Transition>,
    /// Exit/unmount transition
    pub exit: Option<Transition>,
}

/// Standard animation duration values (in seconds)
#[derive(Clone, Debug)]
pub struct AnimationDurations {
    /// Instant/no animation (0s)
    pub instant: f32,
    /// Fast animations (0.1s)
    pub fast: f32,
    /// Normal animations (0.2s)
    pub normal: f32,
    /// Slow animations (0.3s)
    pub slow: f32,
    /// Slower animations (0.5s)
    pub slower: f32,
}

impl Default for AnimationDurations {
    fn default() -> Self {
        Self {
            instant: 0.0,
            fast: 0.1,
            normal: 0.2,
            slow: 0.3,
            slower: 0.5,
        }
    }
}

/// Easing curve settings
#[derive(Clone, Debug)]
pub struct AnimationEasing {
    /// Default easing for enter animations
    pub enter: EasingType,
    /// Default easing for exit animations
    pub exit: EasingType,
    /// Default easing for hover transitions
    pub hover: EasingType,
}

impl Default for AnimationEasing {
    fn default() -> Self {
        Self {
            enter: EasingType::EaseOut,
            exit: EasingType::EaseIn,
            hover: EasingType::EaseInOut,
        }
    }
}

/// Easing curve types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EasingType {
    /// Linear interpolation (no easing)
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in-out (slow start and end)
    EaseInOut,
    /// Cubic ease in
    CubicIn,
    /// Cubic ease out
    CubicOut,
    /// Cubic ease in-out
    CubicInOut,
    /// Spring-like bounce
    Spring,
}

impl EasingType {
    /// Apply the easing function to a value t in [0, 1]
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingType::Linear => t,
            EasingType::EaseIn => t * t,
            EasingType::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingType::CubicIn => t * t * t,
            EasingType::CubicOut => 1.0 - (1.0 - t).powi(3),
            EasingType::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingType::Spring => {
                // Simple spring approximation
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
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
}

/// Default values for hover effects
#[derive(Clone, Debug)]
pub struct HoverDefaults {
    /// Default scale factor on hover (1.0 = no change)
    pub scale: f32,
    /// Default brightness multiplier on hover (1.0 = no change)
    pub brightness: f32,
    /// Animation speed for hover transitions
    pub animation_speed: f32,
}

impl Default for HoverDefaults {
    fn default() -> Self {
        Self {
            scale: 1.08,
            brightness: 1.3,
            animation_speed: 12.0,
        }
    }
}

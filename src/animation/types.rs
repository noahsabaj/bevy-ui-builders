//! Core animation types for the UI system.

use bevy::prelude::*;

use super::effects::HoverEffect;
use super::transitions::Transition;

/// Unified animation component for UI elements.
///
/// This component provides a complete animation solution including:
/// - Hover/interaction state transitions
/// - Enter/exit animations
/// - Custom animation targets
///
/// # Example
///
/// ```ignore
/// // Add to an entity for automatic animation
/// commands.spawn((
///     Button,
///     UiAnimation::default()
///         .with_hover_scale(1.05)
///         .with_hover_brightness(1.1),
/// ));
/// ```
#[derive(Component, Clone, Debug)]
pub struct UiAnimation {
    /// Current animation state
    pub state: AnimationState,
    /// Interaction state transitions (hover, press, etc.)
    pub interaction: InteractionAnimation,
    /// Enter animation (when spawned/shown)
    pub enter: Option<MountAnimation>,
    /// Exit animation (when despawned/hidden)
    pub exit: Option<MountAnimation>,
}

impl Default for UiAnimation {
    fn default() -> Self {
        Self {
            state: AnimationState::default(),
            interaction: InteractionAnimation::default(),
            enter: None,
            exit: None,
        }
    }
}

impl UiAnimation {
    /// Create a new animation with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a hover scale effect
    pub fn with_hover_scale(mut self, scale: f32) -> Self {
        self.interaction.hover.scale = Some(scale);
        self.interaction.pressed.scale = Some(scale * 0.98); // Slightly smaller when pressed
        self
    }

    /// Add a hover brightness effect
    pub fn with_hover_brightness(mut self, brightness: f32) -> Self {
        self.interaction.hover.brightness = Some(brightness);
        self.interaction.pressed.brightness = Some(brightness * 0.95);
        self
    }

    /// Add a hover color shift effect
    pub fn with_hover_color(mut self, color: Color) -> Self {
        self.interaction.hover.color_target = Some(color);
        self
    }

    /// Set the animation speed
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.state.animation_speed = speed;
        self
    }

    /// Add an enter animation
    pub fn with_enter(mut self, transition: Transition) -> Self {
        self.enter = Some(MountAnimation::from_transition(transition));
        self
    }

    /// Add an exit animation
    pub fn with_exit(mut self, transition: Transition) -> Self {
        self.exit = Some(MountAnimation::from_transition(transition));
        self
    }

    /// Apply a preset animation configuration
    pub fn with_preset(mut self, preset: AnimationPreset) -> Self {
        match preset {
            AnimationPreset::None => {
                self.interaction = InteractionAnimation::default();
            }
            AnimationPreset::Subtle => {
                self.interaction.hover.scale = Some(1.02);
                self.interaction.hover.brightness = Some(1.05);
                self.interaction.pressed.scale = Some(0.99);
            }
            AnimationPreset::Punchy => {
                self.interaction.hover.scale = Some(1.05);
                self.interaction.hover.brightness = Some(1.15);
                self.interaction.pressed.scale = Some(0.97);
            }
            AnimationPreset::Playful => {
                self.interaction.hover.scale = Some(1.08);
                self.interaction.hover.brightness = Some(1.2);
                self.interaction.pressed.scale = Some(0.95);
                self.state.animation_speed = 15.0; // Faster, bouncier
            }
            AnimationPreset::Lift => {
                self.interaction.hover.scale = Some(1.02);
                self.interaction.hover.translation = Some(Vec2::new(0.0, -2.0));
            }
            AnimationPreset::Glow => {
                self.interaction.hover.brightness = Some(1.3);
                self.interaction.hover.border_glow = Some(1.5);
            }
        }
        self
    }

    /// Create from category defaults (theme integration)
    pub fn from_category_defaults(defaults: &crate::theme::CategoryDefaults) -> Self {
        let mut anim = Self::new();

        // Set hover effects
        if defaults.hover_scale != 1.0 {
            anim.interaction.hover.scale = Some(defaults.hover_scale);
        }
        if defaults.hover_brightness != 1.0 {
            anim.interaction.hover.brightness = Some(defaults.hover_brightness);
        }

        // Set pressed effects
        if defaults.press_scale != 1.0 {
            anim.interaction.pressed.scale = Some(defaults.press_scale);
        }

        // Set animation speed
        anim.state.animation_speed = defaults.animation_speed;

        // Set enter/exit transitions
        if let Some(ref transition) = defaults.enter {
            anim.enter = Some(MountAnimation::from_transition(transition.clone()));
        }
        if let Some(ref transition) = defaults.exit {
            anim.exit = Some(MountAnimation::from_transition(transition.clone()));
        }

        anim
    }

    /// Create from hover effects (builder integration)
    pub fn from_effects(effects: &[HoverEffect]) -> Self {
        let mut anim = Self::default();
        for effect in effects {
            match effect {
                HoverEffect::Scale(s) => {
                    anim.interaction.hover.scale = Some(*s);
                    anim.interaction.pressed.scale = Some(s * 0.98);
                }
                HoverEffect::Brightness(b) => {
                    anim.interaction.hover.brightness = Some(*b);
                }
                HoverEffect::ColorShift { to } => {
                    anim.interaction.hover.color_target = Some(*to);
                }
                HoverEffect::BorderGlow { color, width } => {
                    anim.interaction.hover.border_color = Some(*color);
                    anim.interaction.hover.border_glow = Some(*width);
                }
                HoverEffect::Lift { distance } => {
                    anim.interaction.hover.translation = Some(Vec2::new(0.0, -*distance));
                }
                HoverEffect::Opacity(o) => {
                    anim.interaction.hover.opacity = Some(*o);
                }
            }
        }
        anim
    }
}

/// Current state of an animation
#[derive(Clone, Debug)]
pub struct AnimationState {
    /// Current scale (1.0 = normal)
    pub current_scale: f32,
    /// Target scale
    pub target_scale: f32,
    /// Current brightness multiplier (1.0 = normal)
    pub current_brightness: f32,
    /// Target brightness
    pub target_brightness: f32,
    /// Current opacity (1.0 = fully visible)
    pub current_opacity: f32,
    /// Target opacity
    pub target_opacity: f32,
    /// Current color blend factor (0.0 = original, 1.0 = target color)
    pub current_color_blend: f32,
    /// Target color blend
    pub target_color_blend: f32,
    /// Current translation offset
    pub current_translation: Vec2,
    /// Target translation
    pub target_translation: Vec2,
    /// Animation speed multiplier
    pub animation_speed: f32,
    /// Whether the element is currently animating
    pub is_animating: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            current_brightness: 1.0,
            target_brightness: 1.0,
            current_opacity: 1.0,
            target_opacity: 1.0,
            current_color_blend: 0.0,
            target_color_blend: 0.0,
            current_translation: Vec2::ZERO,
            target_translation: Vec2::ZERO,
            animation_speed: 10.0,
            is_animating: false,
        }
    }
}

impl AnimationState {
    /// Check if we're close to target (animation complete)
    pub fn is_complete(&self) -> bool {
        (self.current_scale - self.target_scale).abs() < 0.001
            && (self.current_brightness - self.target_brightness).abs() < 0.001
            && (self.current_opacity - self.target_opacity).abs() < 0.001
            && (self.current_color_blend - self.target_color_blend).abs() < 0.001
            && self.current_translation.distance(self.target_translation) < 0.1
    }

    /// Reset to default state
    pub fn reset(&mut self) {
        self.target_scale = 1.0;
        self.target_brightness = 1.0;
        self.target_opacity = 1.0;
        self.target_color_blend = 0.0;
        self.target_translation = Vec2::ZERO;
    }
}

/// Animation configuration for interaction states
#[derive(Clone, Debug, Default)]
pub struct InteractionAnimation {
    /// Animation when idle (not hovered)
    pub idle: AnimationTarget,
    /// Animation when hovered
    pub hover: AnimationTarget,
    /// Animation when pressed
    pub pressed: AnimationTarget,
    /// Animation when focused
    pub focused: AnimationTarget,
    /// Animation when disabled
    pub disabled: AnimationTarget,
}

/// Target values for an animation state
#[derive(Clone, Debug, Default)]
pub struct AnimationTarget {
    /// Scale factor (None = no change)
    pub scale: Option<f32>,
    /// Brightness multiplier (None = no change)
    pub brightness: Option<f32>,
    /// Opacity (None = no change)
    pub opacity: Option<f32>,
    /// Target color to blend towards (None = no change)
    pub color_target: Option<Color>,
    /// Border color override
    pub border_color: Option<Color>,
    /// Border glow intensity
    pub border_glow: Option<f32>,
    /// Translation offset
    pub translation: Option<Vec2>,
}

impl AnimationTarget {
    /// Check if this target has any effects
    pub fn has_effects(&self) -> bool {
        self.scale.is_some()
            || self.brightness.is_some()
            || self.opacity.is_some()
            || self.color_target.is_some()
            || self.border_color.is_some()
            || self.translation.is_some()
    }
}

/// Animation for mount/unmount (enter/exit)
#[derive(Clone, Debug)]
pub struct MountAnimation {
    /// The transition to use
    pub transition: Transition,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Whether the animation is playing
    pub playing: bool,
    /// Whether this is a reverse (exit) animation
    pub reverse: bool,
}

impl MountAnimation {
    /// Create from a transition
    pub fn from_transition(transition: Transition) -> Self {
        Self {
            transition,
            progress: 0.0,
            playing: false,
            reverse: false,
        }
    }

    /// Start playing the animation
    pub fn play(&mut self) {
        self.progress = 0.0;
        self.playing = true;
        self.reverse = false;
    }

    /// Start playing in reverse (for exit)
    pub fn play_reverse(&mut self) {
        self.progress = 1.0;
        self.playing = true;
        self.reverse = true;
    }

    /// Check if complete
    pub fn is_complete(&self) -> bool {
        if self.reverse {
            self.progress <= 0.0
        } else {
            self.progress >= 1.0
        }
    }
}

/// Preset animation configurations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AnimationPreset {
    /// No animation effects
    #[default]
    None,
    /// Subtle, professional animation (scale 1.02, brightness 1.05)
    Subtle,
    /// More pronounced, satisfying feedback (scale 1.05, brightness 1.15)
    Punchy,
    /// Bouncy, game-like animations (scale 1.08, faster)
    Playful,
    /// Lift up effect with subtle shadow
    Lift,
    /// Glow/brightness effect
    Glow,
}

/// Marker component for elements currently in enter animation
#[derive(Component)]
pub struct EnterAnimating;

/// Marker component for elements currently in exit animation
#[derive(Component)]
pub struct ExitAnimating;

/// Component to store original visual properties for animation restoration
#[derive(Component, Clone, Debug)]
pub struct AnimationOriginals {
    /// Original background color
    pub background: Option<Color>,
    /// Original border color
    pub border: Option<Color>,
    /// Original scale
    pub scale: Vec3,
    /// Original translation
    pub translation: Vec3,
}

impl Default for AnimationOriginals {
    fn default() -> Self {
        Self {
            background: None,
            border: None,
            scale: Vec3::ONE,
            translation: Vec3::ZERO,
        }
    }
}

/// Category marker for per-category animation defaults.
///
/// Different UI component categories have different optimal animation settings.
/// This component tells the auto-add system which defaults to apply.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AnimationCategory {
    /// Buttons, checkboxes, radio buttons - scale 1.02, brightness 1.05
    #[default]
    Button,
    /// Sliders, range inputs - scale 1.15 for handle
    Slider,
    /// Text inputs, dropdowns - no scale, focus effects
    Input,
    /// Panels, cards, containers - subtle fade-in
    Container,
    /// Dialogs, modals, overlays - fade-scale enter/exit
    Overlay,
}

/// Marker component to disable automatic animation addition.
///
/// Add this to entities that should not receive auto-animation even if they
/// have an `Interaction` component and `AnimationCategory`.
///
/// # Example
///
/// ```ignore
/// ButtonBuilder::new("No Animation")
///     .no_animation()  // Adds DisableAutoAnimation
///     .build(parent);
/// ```
#[derive(Component, Default)]
pub struct DisableAutoAnimation;

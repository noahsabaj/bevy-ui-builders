//! Button component types and markers

use bevy::prelude::*;

/// Marker component for styled buttons
#[derive(Component)]
pub struct StyledButton;

/// Component storing button-specific state colors
/// This is kept for compatibility with existing button code
/// but new code should use HoverColors from systems::hover
#[derive(Component)]
pub struct ButtonStateColors {
    pub normal_bg: Color,
    pub hover_bg: Color,
    pub pressed_bg: Color,
    pub normal_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

/// Component for button-specific animation state
/// This is kept for compatibility with existing button code
/// but new code should use HoverAnimationState from systems::hover
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

// Note: HoverScale, HoverBrightness, and OriginalColors have been moved to
// systems::hover for universal use across all UI elements

// ============================================================================
// Selection State Components
// ============================================================================

/// Marker component indicating this button supports selection/toggle behavior.
/// Buttons with this component will automatically respond to clicks by toggling
/// their Selected state (if auto_toggle is true).
#[derive(Component)]
pub struct SelectableButton {
    /// Whether the button auto-toggles Selected on click (default: true)
    pub auto_toggle: bool,
}

impl Default for SelectableButton {
    fn default() -> Self {
        Self { auto_toggle: true }
    }
}

/// Marker component indicating button is currently selected.
/// Used for toggle buttons, checkboxes, or items in a list.
/// Can be added/removed at runtime to change selection state.
#[derive(Component)]
pub struct Selected;

/// Marker component indicating button is currently active.
/// Used for current tab in a tab bar, current page in navigation, etc.
/// Active state takes visual precedence over Selected state.
/// Can coexist with Selected component.
#[derive(Component)]
pub struct Active;

/// Color configuration for all button states.
/// Auto-generated from ButtonStyle by default, but can be overridden.
#[derive(Component, Clone)]
pub struct ButtonSelectionColors {
    pub normal: StateColorSet,
    pub selected: StateColorSet,
    pub active: StateColorSet,
}

/// Color set for a specific state (normal/selected/active).
/// Contains colors for all interaction states (normal/hover/pressed).
#[derive(Clone, Debug)]
pub struct StateColorSet {
    pub normal_bg: Color,
    pub hover_bg: Color,
    pub pressed_bg: Color,
    pub normal_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

impl StateColorSet {
    /// Create a new StateColorSet with the given colors
    pub fn new(
        normal_bg: Color,
        hover_bg: Color,
        pressed_bg: Color,
        normal_border: Color,
        hover_border: Color,
        pressed_border: Color,
    ) -> Self {
        Self {
            normal_bg,
            hover_bg,
            pressed_bg,
            normal_border,
            hover_border,
            pressed_border,
        }
    }

    /// Create a StateColorSet from a single background and border color
    /// (generates hover/pressed variants automatically)
    pub fn from_base(bg: Color, border: Color) -> Self {
        Self {
            normal_bg: bg,
            hover_bg: adjust_brightness(bg, 1.1),
            pressed_bg: adjust_brightness(bg, 0.9),
            normal_border: border,
            hover_border: border,
            pressed_border: border,
        }
    }
}

/// Message emitted when a selectable button's selection state changes
#[derive(Message, Clone, Debug)]
pub struct SelectionChanged {
    /// The button entity that changed
    pub entity: Entity,
    /// Whether the button is now selected
    pub selected: bool,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Adjust brightness of a color by a multiplier
fn adjust_brightness(color: Color, multiplier: f32) -> Color {
    let linear = color.to_linear();
    Color::LinearRgba(LinearRgba {
        red: (linear.red * multiplier).min(1.0),
        green: (linear.green * multiplier).min(1.0),
        blue: (linear.blue * multiplier).min(1.0),
        alpha: linear.alpha,
    })
}
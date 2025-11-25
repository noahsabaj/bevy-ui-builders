//! ScrollView component types

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Marker component for scroll view containers
#[derive(Component, Debug, Clone, Default)]
pub struct ScrollView;

/// Resolved scrollbar colors (used by builder)
pub struct ScrollbarColors {
    /// Track (background) color
    pub track: Color,
    /// Thumb (draggable) color
    pub thumb: Color,
}

impl ScrollbarColors {
    /// Resolve scrollbar colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        use bevy::color::Alpha;
        Self {
            track: theme.colors.border.light,
            thumb: theme.colors.border.default.with_alpha(0.5),
        }
    }

    /// Default scrollbar colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            track: Color::srgba(1.0, 1.0, 1.0, 0.1),
            thumb: Color::srgba(1.0, 1.0, 1.0, 0.3),
        }
    }
}

/// Configuration for scroll behavior
#[derive(Component, Debug, Clone)]
pub struct ScrollConfig {
    /// Whether to auto-scroll to focused elements
    pub auto_scroll_to_focus: bool,
    /// Scrollbar visibility mode
    pub scrollbar_visibility: ScrollbarVisibility,
    /// Enable drag-to-scroll (click and drag to scroll)
    pub enable_drag_scroll: bool,
    /// Enable kinetic scrolling (momentum after drag)
    pub enable_kinetic_scroll: bool,
    /// Scroll sensitivity multiplier for mouse wheel
    pub scroll_sensitivity: f32,
    /// Minimum scrollbar thumb length in pixels
    pub min_thumb_length: f32,
    /// Scrollbar width in pixels
    pub scrollbar_width: f32,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            auto_scroll_to_focus: true,
            scrollbar_visibility: ScrollbarVisibility::AutoHide { timeout_secs: 2.0 },
            enable_drag_scroll: true,
            enable_kinetic_scroll: true,
            scroll_sensitivity: 1.0,
            min_thumb_length: 8.0,
            scrollbar_width: 8.0,
        }
    }
}

/// Scrollbar visibility modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollbarVisibility {
    /// Always visible
    Always,
    /// Auto-hide after timeout
    AutoHide {
        /// Seconds of inactivity before hiding
        timeout_secs: f32
    },
    /// Only show on hover
    OnHover,
    /// Never show scrollbars
    Never,
}

/// Direction of scrolling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    /// Vertical scrolling only
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both directions
    Both,
}

/// Tracks scrollbar visibility state
#[derive(Component, Debug, Clone)]
pub struct ScrollbarState {
    /// Time since last scroll interaction
    pub time_since_interaction: f32,
    /// Current opacity for fade animations
    pub opacity: f32,
    /// Target scroll container entity
    pub scroll_container: Entity,
}

impl ScrollbarState {
    /// Create a new scrollbar state for a container
    pub fn new(container: Entity) -> Self {
        Self {
            time_since_interaction: 999.0, // Start hidden
            opacity: 0.0,
            scroll_container: container,
        }
    }
}

/// Tracks kinetic scrolling state
#[derive(Component, Debug, Clone, Default)]
pub struct KineticScrollState {
    /// Current velocity in pixels per second
    pub velocity: Vec2,
    /// Whether kinetic scrolling is active
    pub active: bool,
    /// Last drag position for velocity calculation
    pub last_position: Option<Vec2>,
    /// Last drag timestamp
    pub last_time: Option<f32>,
}

/// Marker for drag-scrollable containers
#[derive(Component, Debug, Clone, Default)]
pub struct DragScrollTarget;

/// Visual indicator for scroll availability
#[derive(Component, Debug, Clone)]
pub struct ScrollIndicator {
    /// Position of the indicator
    pub position: IndicatorPosition,
    /// Whether the indicator is currently visible
    pub visible: bool,
}

/// Position of scroll indicators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndicatorPosition {
    /// Top edge
    Top,
    /// Bottom edge
    Bottom,
    /// Left edge
    Left,
    /// Right edge
    Right,
}

//! ScrollView component types

use bevy::prelude::*;
use bevy::ui::ComputedNode;

/// Marker component for scroll view containers
#[derive(Component, Debug, Clone, Default)]
pub struct ScrollView;

/// Tracks the current scroll state and limits
#[derive(Component, Debug, Clone)]
pub struct ScrollState {
    /// Current scroll offset in pixels
    pub offset: Vec2,
    /// Maximum scroll offset based on content size
    pub max_offset: Vec2,
    /// Target offset for smooth scrolling
    pub target_offset: Option<Vec2>,
    /// Time since last scroll (for auto-hide scrollbars)
    pub last_scroll_time: f32,
}

impl Default for ScrollState {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            max_offset: Vec2::ZERO,
            target_offset: None,
            last_scroll_time: 999.0, // Start hidden
        }
    }
}

/// Configuration for scroll behavior
#[derive(Component, Debug, Clone)]
pub struct ScrollConfig {
    /// Whether to auto-scroll to focused elements
    pub auto_scroll_to_focus: bool,
    /// Scroll animation duration in seconds
    pub animation_duration: f32,
    /// Show scroll indicators
    pub show_indicators: bool,
    /// Padding inside the scroll container (as viewport percentage)
    pub padding_vh: f32,
    /// Scroll sensitivity multiplier
    pub sensitivity: f32,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            auto_scroll_to_focus: true,
            animation_duration: 0.3,
            show_indicators: true,
            padding_vh: 2.0,
            sensitivity: 1.0,
        }
    }
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

/// Component for scrollbar track
#[derive(Component, Debug, Clone, Default)]
pub struct ScrollBarTrack;

/// Component for scrollbar thumb
#[derive(Component, Debug, Clone)]
pub struct ScrollBarThumb {
    /// Reference to the scroll container this thumb controls
    pub scroll_container: Entity,
}

/// Visual indicator for scroll availability
#[derive(Component, Debug, Clone)]
pub struct ScrollIndicator {
    /// Position of the indicator
    pub position: IndicatorPosition,
    /// Whether the indicator is currently visible
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndicatorPosition {
    Top,
    Bottom,
    Left,
    Right,
}
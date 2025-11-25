//! Tooltip types and components

use bevy::prelude::*;
use std::time::Duration;
use crate::traits::TooltipPosition;

/// Component that marks an entity as having a tooltip
#[derive(Component, Clone, Debug)]
pub struct HasTooltip {
    /// The tooltip content
    pub content: TooltipContent,
    /// Position preference
    pub position: TooltipPosition,
    /// Delay before showing (in seconds)
    pub delay: Duration,
    /// Maximum width of tooltip
    pub max_width: f32,
}

/// The content of a tooltip
#[derive(Clone, Debug)]
pub enum TooltipContent {
    /// Simple text tooltip
    Simple(String),
    /// Rich tooltip with title and description
    Rich {
        title: String,
        description: Option<String>,
    },
}

impl TooltipContent {
    /// Get the title text
    pub fn title(&self) -> &str {
        match self {
            TooltipContent::Simple(text) => text,
            TooltipContent::Rich { title, .. } => title,
        }
    }

    /// Get the description text if available
    pub fn description(&self) -> Option<&str> {
        match self {
            TooltipContent::Simple(_) => None,
            TooltipContent::Rich { description, .. } => description.as_deref(),
        }
    }
}

/// Marker component for tooltip UI entities
#[derive(Component, Clone, Debug)]
pub struct TooltipEntity {
    /// The entity this tooltip belongs to
    pub target: Entity,
}

/// State tracking for tooltip visibility
#[derive(Component, Clone, Debug, Default)]
pub struct TooltipState {
    /// Whether the tooltip is currently visible
    pub visible: bool,
    /// Time spent hovering (for delay calculation)
    pub hover_time: f32,
    /// The spawned tooltip entity (if visible)
    pub tooltip_entity: Option<Entity>,
}

/// Resource for global tooltip configuration
#[derive(Resource, Clone, Debug)]
pub struct TooltipSettings {
    /// Default delay before showing tooltips
    pub default_delay: Duration,
    /// Default maximum width
    pub default_max_width: f32,
    /// Z-index for tooltips
    pub z_index: i32,
    /// Offset from cursor/element
    pub offset: f32,
}

impl Default for TooltipSettings {
    fn default() -> Self {
        Self {
            default_delay: Duration::from_millis(500),
            default_max_width: 300.0,
            z_index: 2000,
            offset: 8.0,
        }
    }
}

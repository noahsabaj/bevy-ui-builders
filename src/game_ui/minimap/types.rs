//! Minimap types and components

use bevy::prelude::*;

/// Component marking a minimap
#[derive(Component, Clone, Debug)]
pub struct Minimap {
    /// Size of the minimap
    pub size: f32,
    /// Zoom level
    pub zoom: f32,
    /// Whether to rotate with player
    pub rotate_with_player: bool,
}

/// Shape of the minimap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MinimapShape {
    /// Circular minimap
    #[default]
    Circle,
    /// Square minimap
    Square,
    /// Rounded square
    RoundedSquare,
}

/// Rotation mode for the minimap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MinimapRotation {
    /// Fixed north orientation
    #[default]
    Fixed,
    /// Rotate with player
    FollowPlayer,
}

/// Component for the minimap content area
#[derive(Component, Clone, Debug)]
pub struct MinimapContent {
    /// The minimap this belongs to
    pub minimap: Entity,
}

/// Component for minimap markers/icons
#[derive(Component, Clone, Debug)]
pub struct MinimapMarker {
    /// The minimap this marker belongs to
    pub minimap: Entity,
    /// World position of the marker
    pub world_position: Vec2,
    /// Marker type
    pub marker_type: MarkerType,
}

/// Types of minimap markers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerType {
    /// Player marker
    #[default]
    Player,
    /// Enemy marker
    Enemy,
    /// Ally/NPC marker
    Ally,
    /// Objective marker
    Objective,
    /// Point of interest
    PointOfInterest,
    /// Custom marker
    Custom,
}

impl MarkerType {
    /// Get the default color for this marker type
    pub fn color(&self) -> Color {
        match self {
            Self::Player => Color::srgb(0.2, 0.8, 0.2),
            Self::Enemy => Color::srgb(0.9, 0.2, 0.2),
            Self::Ally => Color::srgb(0.2, 0.6, 0.9),
            Self::Objective => Color::srgb(0.9, 0.8, 0.2),
            Self::PointOfInterest => Color::srgb(0.8, 0.8, 0.8),
            Self::Custom => Color::WHITE,
        }
    }
}

/// Resource for minimap settings
#[derive(Resource, Clone, Debug)]
pub struct MinimapSettings {
    /// Default zoom level
    pub default_zoom: f32,
    /// Update rate (times per second)
    pub update_rate: f32,
}

impl Default for MinimapSettings {
    fn default() -> Self {
        Self {
            default_zoom: 1.0,
            update_rate: 30.0,
        }
    }
}

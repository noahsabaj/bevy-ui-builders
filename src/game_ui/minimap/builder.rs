//! MinimapBuilder implementation

use bevy::prelude::*;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating minimaps
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::game_ui::minimap::*;
///
/// // Basic circular minimap
/// MinimapBuilder::new()
///     .size(200.0)
///     .shape(MinimapShape::Circle)
///     .build(parent);
///
/// // Square minimap that rotates with player
/// MinimapBuilder::new()
///     .size(150.0)
///     .shape(MinimapShape::Square)
///     .rotation(MinimapRotation::FollowPlayer)
///     .zoom(1.5)
///     .build(parent);
/// ```
pub struct MinimapBuilder {
    size: f32,
    shape: MinimapShape,
    rotation: MinimapRotation,
    zoom: f32,
    show_border: bool,
    border_width: f32,
    background_color: Color,
    border_color: Color,
    base: BuilderBase,
}

impl MinimapBuilder {
    /// Create a new minimap builder
    pub fn new() -> Self {
        Self {
            size: 200.0,
            shape: MinimapShape::Circle,
            rotation: MinimapRotation::Fixed,
            zoom: 1.0,
            show_border: true,
            border_width: 3.0,
            background_color: Color::srgba(0.1, 0.1, 0.12, 0.9),
            border_color: Color::srgb(0.4, 0.4, 0.4),
            base: BuilderBase::new(),
        }
    }

    /// Set the minimap size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the minimap shape
    pub fn shape(mut self, shape: MinimapShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set the rotation mode
    pub fn rotation(mut self, rotation: MinimapRotation) -> Self {
        self.rotation = rotation;
        self
    }

    /// Set the zoom level
    pub fn zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    /// Show or hide the border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set the border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
}

impl Default for MinimapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for MinimapBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let border_radius = match self.shape {
            MinimapShape::Circle => Val::Percent(50.0),
            MinimapShape::Square => Val::Px(0.0),
            MinimapShape::RoundedSquare => Val::Px(12.0),
        };

        let minimap_entity = parent.spawn_empty().id();

        parent.commands().entity(minimap_entity).insert((
            Node {
                width: Val::Px(self.size),
                height: Val::Px(self.size),
                border: if self.show_border {
                    UiRect::all(Val::Px(self.border_width))
                } else {
                    UiRect::ZERO
                },
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(self.background_color),
            BorderColor::all(self.border_color),
            BorderRadius::all(border_radius),
            Minimap {
                size: self.size,
                zoom: self.zoom,
                rotate_with_player: matches!(self.rotation, MinimapRotation::FollowPlayer),
            },
        )).with_children(|minimap| {
            // Content area (for markers)
            minimap.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                MinimapContent {
                    minimap: minimap_entity,
                },
            ));
        });

        minimap_entity
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for MinimapBuilder {
    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(self, _padding: UiRect) -> Self {
        self
    }
}

/// Convenience function to create a minimap builder
pub fn minimap() -> MinimapBuilder {
    MinimapBuilder::new()
}

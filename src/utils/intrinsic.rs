//! Intrinsic sizing utilities - Let content determine its own size
//!
//! Following the philosophy that content knows best, these utilities
//! help create layouts that adapt to their content rather than
//! forcing fixed dimensions.

use bevy::prelude::*;

/// Create a node that sizes itself based on content
#[allow(dead_code)]
pub fn intrinsic_node() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Stretch,
        ..default()
    }
}

/// Create a scrollable container with intrinsic height
#[allow(dead_code)]
pub fn scrollable_container(max_height: Val) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        max_height,
        overflow: Overflow::scroll_y(),
        ..default()
    }
}

/// Create a panel that adapts to content with safety constraints
#[allow(dead_code)]
pub fn adaptive_panel(max_width: Val, max_height: Val) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        max_width,
        max_height,
        overflow: Overflow::clip(),
        ..default()
    }
}
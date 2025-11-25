//! Layout helper builders
//!
//! Provides convenient builders for common layout patterns.

use bevy::prelude::*;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

// ============================================================================
// UiContainer - Base bundle for UI hierarchy nodes
// ============================================================================

/// A bundle for UI container nodes that ensures proper hierarchy component setup.
///
/// In Bevy 0.17, the B0004 hierarchy check can run before required components
/// (like GlobalTransform from Node) are fully applied. This bundle explicitly
/// includes Transform to prevent these timing-related warnings.
///
/// # Example
/// ```ignore
/// // Instead of:
/// parent.spawn(Node { ... }).with_children(|p| { ... });
///
/// // Use:
/// parent.spawn(UiContainer::new(Node { ... })).with_children(|p| { ... });
/// // Or with the builder:
/// parent.spawn(UiContainer::row().gap(8.0).build()).with_children(|p| { ... });
/// ```
#[derive(Bundle, Clone, Default)]
pub struct UiContainer {
    /// The UI node configuration
    pub node: Node,
    /// Transform required for proper hierarchy (prevents B0004 warnings)
    pub transform: Transform,
}

impl UiContainer {
    /// Create a new UI container with the given node configuration
    pub fn new(node: Node) -> Self {
        Self {
            node,
            transform: Transform::default(),
        }
    }

    /// Create a row container (horizontal layout)
    pub fn row() -> UiContainerBuilder {
        UiContainerBuilder::new().direction(FlexDirection::Row)
    }

    /// Create a column container (vertical layout)
    pub fn column() -> UiContainerBuilder {
        UiContainerBuilder::new().direction(FlexDirection::Column)
    }

    /// Create a centered full-screen container
    pub fn centered() -> Self {
        Self::new(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
    }

    /// Create a full-screen container
    pub fn fullscreen() -> Self {
        Self::new(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
    }
}

/// Builder for UiContainer with fluent API
#[derive(Clone)]
pub struct UiContainerBuilder {
    node: Node,
    background: Option<Color>,
    border_color: Option<Color>,
    border_radius: Option<Val>,
}

impl UiContainerBuilder {
    /// Create a new container builder
    pub fn new() -> Self {
        Self {
            node: Node::default(),
            background: None,
            border_color: None,
            border_radius: None,
        }
    }

    /// Set the flex direction
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.node.flex_direction = direction;
        self
    }

    /// Set the gap between items (row_gap for columns, column_gap for rows)
    pub fn gap(mut self, gap: Val) -> Self {
        match self.node.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.node.column_gap = gap;
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.node.row_gap = gap;
            }
        }
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.node.padding = padding;
        self
    }

    /// Set uniform padding
    pub fn padding_all(mut self, padding: Val) -> Self {
        self.node.padding = UiRect::all(padding);
        self
    }

    /// Set margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.node.margin = margin;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: Val) -> Self {
        self.node.height = height;
        self
    }

    /// Set justify content
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.node.justify_content = justify;
        self
    }

    /// Set align items
    pub fn align(mut self, align: AlignItems) -> Self {
        self.node.align_items = align;
        self
    }

    /// Center content
    pub fn center(mut self) -> Self {
        self.node.justify_content = JustifyContent::Center;
        self.node.align_items = AlignItems::Center;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set border
    pub fn border(mut self, width: Val, color: Color) -> Self {
        self.node.border = UiRect::all(width);
        self.border_color = Some(color);
        self
    }

    /// Set border radius
    pub fn border_radius(mut self, radius: Val) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Build just the UiContainer bundle
    pub fn build(self) -> UiContainer {
        UiContainer::new(self.node)
    }

    /// Build and spawn as a child, returning EntityCommands for chaining
    pub fn spawn(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let mut entity = parent.spawn(UiContainer::new(self.node));

        if let Some(bg) = self.background {
            entity.insert(BackgroundColor(bg));
        }
        if let Some(border) = self.border_color {
            entity.insert(BorderColor::all(border));
        }
        if let Some(radius) = self.border_radius {
            entity.insert(BorderRadius::all(radius));
        }

        entity.id()
    }
}

impl Default for UiContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Existing Layout Builders (updated to include Transform)
// ============================================================================

/// Builder for creating horizontal row layouts
pub struct RowBuilder {
    gap: Val,
    justify: JustifyContent,
    align: AlignItems,
    wrap: FlexWrap,
    base: BuilderBase,
}

impl RowBuilder {
    /// Create a new row builder
    pub fn new() -> Self {
        Self {
            gap: Val::Px(8.0),
            justify: JustifyContent::FlexStart,
            align: AlignItems::Center,
            wrap: FlexWrap::NoWrap,
            base: BuilderBase::new(),
        }
    }

    /// Set the gap between items
    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    /// Set horizontal justification
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    /// Set vertical alignment
    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    /// Center items both horizontally and vertically
    pub fn center(mut self) -> Self {
        self.justify = JustifyContent::Center;
        self.align = AlignItems::Center;
        self
    }

    /// Distribute items with space between
    pub fn space_between(mut self) -> Self {
        self.justify = JustifyContent::SpaceBetween;
        self
    }

    /// Distribute items with space around
    pub fn space_around(mut self) -> Self {
        self.justify = JustifyContent::SpaceAround;
        self
    }

    /// Align items to the end
    pub fn end(mut self) -> Self {
        self.justify = JustifyContent::FlexEnd;
        self
    }

    /// Allow wrapping
    pub fn wrap(mut self) -> Self {
        self.wrap = FlexWrap::Wrap;
        self
    }
}

impl Default for RowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for RowBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        self.base.node.flex_direction = FlexDirection::Row;
        self.base.node.column_gap = self.gap;
        self.base.node.justify_content = self.justify;
        self.base.node.align_items = self.align;
        self.base.node.flex_wrap = self.wrap;

        // Include Transform to prevent B0004 warnings when using with_children
        let entity = parent
            .spawn(UiContainer::new(self.base.node.clone()))
            .id();

        self.base.apply(entity, &mut parent.commands());
        entity
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

impl LayoutBuilder for RowBuilder {
    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }
}

/// Builder for creating vertical column layouts
pub struct ColumnBuilder {
    gap: Val,
    justify: JustifyContent,
    align: AlignItems,
    base: BuilderBase,
}

impl ColumnBuilder {
    /// Create a new column builder
    pub fn new() -> Self {
        Self {
            gap: Val::Px(8.0),
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            base: BuilderBase::new(),
        }
    }

    /// Set the gap between items
    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    /// Set vertical justification
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    /// Set horizontal alignment
    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    /// Center items both horizontally and vertically
    pub fn center(mut self) -> Self {
        self.justify = JustifyContent::Center;
        self.align = AlignItems::Center;
        self
    }

    /// Distribute items with space between
    pub fn space_between(mut self) -> Self {
        self.justify = JustifyContent::SpaceBetween;
        self
    }

    /// Align items to the end
    pub fn end(mut self) -> Self {
        self.justify = JustifyContent::FlexEnd;
        self
    }
}

impl Default for ColumnBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for ColumnBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        self.base.node.flex_direction = FlexDirection::Column;
        self.base.node.row_gap = self.gap;
        self.base.node.justify_content = self.justify;
        self.base.node.align_items = self.align;

        // Include Transform to prevent B0004 warnings when using with_children
        let entity = parent
            .spawn(UiContainer::new(self.base.node.clone()))
            .id();

        self.base.apply(entity, &mut parent.commands());
        entity
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

impl LayoutBuilder for ColumnBuilder {
    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }
}

// Convenience functions

/// Create a horizontal row layout
pub fn row() -> RowBuilder {
    RowBuilder::new()
}

/// Create a vertical column layout
pub fn column() -> ColumnBuilder {
    ColumnBuilder::new()
}

/// Create a centered container
pub fn centered() -> ColumnBuilder {
    ColumnBuilder::new()
        .center()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
}

/// Create a spacer that grows to fill available space
pub fn spacer() -> SpacerBuilder {
    SpacerBuilder::new()
}

/// Builder for flexible spacers
pub struct SpacerBuilder {
    grow: f32,
}

impl SpacerBuilder {
    pub fn new() -> Self {
        Self { grow: 1.0 }
    }

    /// Set how much this spacer should grow relative to others
    pub fn grow(mut self, factor: f32) -> Self {
        self.grow = factor;
        self
    }
}

impl Default for SpacerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for SpacerBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Include Transform to prevent B0004 warnings when using with_children
        parent
            .spawn(UiContainer::new(Node {
                flex_grow: self.grow,
                ..default()
            }))
            .id()
    }

    fn insert(self, _bundle: impl Bundle + Clone) -> Self {
        self
    }

    fn id(self, _id: Entity) -> Self {
        self
    }
}

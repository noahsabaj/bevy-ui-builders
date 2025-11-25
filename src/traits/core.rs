use bevy::prelude::*;

/// Core functionality for all UI builders.
///
/// This is the base trait that all builders must implement.
/// It provides the essential `build()` method plus utilities
/// for adding components and using pre-spawned entities.
///
/// # Example
///
/// ```ignore
/// // Basic usage
/// ButtonBuilder::new("Click").build(parent);
///
/// // With additional components
/// ButtonBuilder::new("Click")
///     .insert(MyMarkerComponent)
///     .build(parent);
/// ```
pub trait UiBuilder: Sized {
    /// Build the entity and return its ID.
    ///
    /// This consumes the builder and spawns the UI element
    /// as a child of the provided parent.
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity;

    /// Add a component or bundle to the entity being built.
    ///
    /// Components are applied after the entity is spawned.
    fn insert(self, bundle: impl Bundle + Clone) -> Self;

    /// Set a specific entity ID (if pre-spawned).
    ///
    /// Note: This method has a default no-op implementation for backwards compatibility.
    /// Builders should override this if they support pre-spawned entities.
    fn id(self, _id: Entity) -> Self {
        self
    }
}

/// Functionality for builders that have physical layout properties.
///
/// This extends `UiBuilder` with layout-related methods for
/// controlling size, spacing, and positioning.
///
/// All methods have default no-op implementations for backwards compatibility.
/// Builders should override methods they support.
///
/// # Example
///
/// ```ignore
/// PanelBuilder::new()
///     .width(Val::Px(300.0))
///     .height(Val::Px(200.0))
///     .padding(UiRect::all(Val::Px(16.0)))
///     .margin(UiRect::bottom(Val::Px(8.0)))
///     .build(parent);
/// ```
pub trait LayoutBuilder: UiBuilder {
    /// Set the complete node style properties.
    fn node(self, _node: Node) -> Self { self }

    /// Set the width.
    fn width(self, _width: Val) -> Self { self }

    /// Set the height.
    fn height(self, _height: Val) -> Self { self }

    /// Set the minimum width.
    fn min_width(self, _width: Val) -> Self { self }

    /// Set the maximum width.
    fn max_width(self, _width: Val) -> Self { self }

    /// Set the minimum height.
    fn min_height(self, _height: Val) -> Self { self }

    /// Set the maximum height.
    fn max_height(self, _height: Val) -> Self { self }

    /// Set the margin on all sides.
    fn margin(self, _margin: UiRect) -> Self { self }

    /// Set the padding on all sides.
    fn padding(self, _padding: UiRect) -> Self { self }

    /// Set the flex grow factor.
    fn flex_grow(self, _grow: f32) -> Self { self }

    /// Set the flex shrink factor.
    fn flex_shrink(self, _shrink: f32) -> Self { self }

    /// Set the flex basis.
    fn flex_basis(self, _basis: Val) -> Self { self }

    /// Set the aspect ratio.
    fn aspect_ratio(self, _ratio: f32) -> Self { self }
}

/// Base struct for storing common builder data
#[derive(Default)]
pub struct BuilderBase {
    pub entity: Option<Entity>,
    pub node: Node,
    pub hooks: Vec<Box<dyn FnOnce(&mut EntityCommands)>>,
}

impl BuilderBase {
    pub fn new() -> Self {
        Self {
            entity: None,
            node: Node::default(),
            hooks: Vec::new(),
        }
    }

    pub fn apply(&mut self, entity: Entity, commands: &mut Commands) {
        let mut entity_cmds = commands.entity(entity);
        
        // Apply all deferred components
        for hook in self.hooks.drain(..) {
            hook(&mut entity_cmds);
        }
    }
}

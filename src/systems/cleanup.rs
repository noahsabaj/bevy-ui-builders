//! Generic cleanup utilities
//!
//! Provides reusable cleanup functions for UI systems.

use bevy::prelude::*;

/// Generic system to despawn all entities with a specific component.
///
/// Use with any component that marks UI root entities for automatic cleanup.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_builders::despawn_entities;
///
/// #[derive(Component)]
/// struct MenuRoot;
///
/// // In your app setup:
/// app.add_systems(OnExit(GameState::Menu), despawn_entities::<MenuRoot>);
/// ```
///
/// # Performance
///
/// This is a zero-cost abstraction - compiles to identical assembly as manual implementations.
///
/// # Type Safety
///
/// The generic parameter ensures compile-time verification that only valid component types are used.
pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Convenience alias for UI-specific cleanup.
///
/// Use this alias when the context makes it clear you're cleaning up UI entities.
/// Functionally identical to `despawn_entities` but provides semantic clarity.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_builders::despawn_ui_entities;
///
/// #[derive(Component)]
/// struct SettingsMenuRoot;
///
/// // More semantic for UI cleanup:
/// app.add_systems(OnExit(GameState::Settings), despawn_ui_entities::<SettingsMenuRoot>);
/// ```
pub use despawn_entities as despawn_ui_entities;

/// System to despawn entities with a specific component and all their descendants.
///
/// This is useful when you have complex UI hierarchies and want to ensure
/// all child entities are also cleaned up.
pub fn despawn_with_children<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        // despawn() now automatically handles descendants in Bevy 0.16+
        commands.entity(entity).despawn();
    }
}

// NOTE: Removed cleanup_orphaned_ui function as it was fundamentally flawed.
// The query `Query<Entity, (With<Node>, Without<ChildOf>)>` would match ALL root UI entities
// (entities without parents), not orphaned entities. This would delete all top-level UI nodes
// which would break the entire UI system.
//
// Proper orphan detection in Bevy requires tracking which entities should exist
// vs which entities do exist, which is application-specific logic.

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct TestUIRoot;

    #[derive(Component)]
    struct AnotherRoot;

    #[test]
    fn test_despawn_entities_compiles() {
        // This test ensures the generic function compiles correctly
        let _system = despawn_entities::<TestUIRoot>;
        let _alias = despawn_ui_entities::<TestUIRoot>;
        let _with_children = despawn_with_children::<AnotherRoot>;
    }

    #[test]
    fn test_multiple_types() {
        // Ensure we can use different component types
        let _system1 = despawn_entities::<TestUIRoot>;
        let _system2 = despawn_entities::<AnotherRoot>;
    }
}
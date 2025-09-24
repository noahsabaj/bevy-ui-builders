//! Generic cleanup utilities
//!
//! This module eliminates the need for duplicate cleanup functions across UI modules.
//! Instead of writing the same despawn logic in every UI system, use these generic functions.

use bevy::prelude::*;

/// Generic system to despawn all entities with a specific component.
///
/// This function eliminates duplicate cleanup boilerplate across all UI systems.
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
        // despawn() now automatically handles descendants in Bevy 0.16
        commands.entity(entity).despawn();
    }
}

/// Cleanup system that can be scheduled to run periodically to remove orphaned UI entities.
///
/// This is useful as a safety net to catch any UI entities that might have been
/// missed by specific cleanup systems.
pub fn cleanup_orphaned_ui(
    mut commands: Commands,
    query: Query<Entity, (With<Node>, Without<ChildOf>)>,
    camera_query: Query<Entity, With<Camera>>,
) {
    // Don't cleanup if we don't have a camera (game might be shutting down)
    if camera_query.is_empty() {
        return;
    }

    // Despawn any UI nodes that don't have a parent (likely orphaned)
    for entity in &query {
        // Found orphaned UI entity, cleaning up
        commands.entity(entity).despawn();
    }
}

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
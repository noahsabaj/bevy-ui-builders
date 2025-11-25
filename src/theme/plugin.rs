//! Theme plugin for automatic theme resource management.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::types::UiTheme;

define_plugin!(ThemePlugin {
    custom_init: |app: &mut App| {
        // Insert default theme if not already present
        if !app.world().contains_resource::<UiTheme>() {
            app.insert_resource(UiTheme::dark());
        }

        // Register ThemeChanged message
        app.add_message::<ThemeChanged>();
    }
});

/// Message fired when the theme changes.
///
/// Listen for this message to react to theme changes in your systems.
///
/// # Example
///
/// ```ignore
/// fn on_theme_change(
///     mut messages: MessageReader<ThemeChanged>,
///     theme: Res<UiTheme>,
/// ) {
///     for _msg in messages.read() {
///         println!("Theme changed!");
///     }
/// }
/// ```
#[derive(Message, Clone, Debug)]
pub struct ThemeChanged;

/// System to detect theme resource changes and emit ThemeChanged messages.
///
/// Add this system if you need to react to theme changes.
pub fn detect_theme_changes(
    theme: Res<UiTheme>,
    mut messages: MessageWriter<ThemeChanged>,
) {
    if theme.is_changed() && !theme.is_added() {
        messages.write(ThemeChanged);
    }
}

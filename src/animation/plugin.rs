//! Animation plugin for the UI system.

use bevy_plugin_builder::define_plugin;

use super::systems::{
    apply_animation_colors, apply_animation_transforms, auto_add_animation,
    init_animation_originals, process_enter_animations, process_exit_animations,
    tick_animation_states, update_interaction_animations,
};

define_plugin!(AnimationPlugin {
    update: [
        // Auto-add animation to entities with Interaction + AnimationCategory
        auto_add_animation,

        // Initialize originals when animation component is added
        init_animation_originals,

        // Update targets based on interaction changes
        update_interaction_animations,

        // Tick animation states towards targets
        tick_animation_states,

        // Apply animation to visuals
        apply_animation_transforms,
        apply_animation_colors,

        // Process mount/unmount animations
        process_enter_animations,
        process_exit_animations,
    ]
});

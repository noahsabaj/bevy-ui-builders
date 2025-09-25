//! Progress bar plugin

use bevy_plugin_builder::define_plugin;
use super::systems::*;

// Plugin to add progress bar systems
define_plugin!(ProgressBarPlugin {
    update: [update_progress_bars, force_update_progress_bars]
});
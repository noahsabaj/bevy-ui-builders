//! Progress bar plugin

use bevy::prelude::*;
use super::systems::*;

/// Plugin to add progress bar systems
pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_progress_bars, force_update_progress_bars));
    }
}
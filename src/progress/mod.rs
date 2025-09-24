//! Progress bar module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod plugin;
mod systems;
mod types;

// Public exports only
pub use builder::{ProgressBarBuilder, progress};
pub use plugin::ProgressBarPlugin;
pub use systems::{update_progress_bars, force_update_progress_bars};
pub use types::{
    ProgressBar,
    ProgressBarStyle,
    ProgressBarFill,
    ProgressBarTrack,
    ProgressBarLabel,
};
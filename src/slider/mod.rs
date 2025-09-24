//! Slider module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod plugin;
mod systems;
mod types;

// Public exports only
pub use builder::SliderBuilder;
pub use plugin::SliderPlugin;
pub use types::{
    Slider,
    SliderHandle,
    SliderTrack,
    SliderFill,
    SliderValueText,
    SliderLabel,
    SliderButtonAction,
    SliderConfig,
    ValueFormat,
};
//! ScrollView module - Gateway only

// Private submodules
mod builder;
mod types;
mod plugin;
mod systems;

// Public exports
pub use builder::{ScrollViewBuilder, scroll_view};
pub use types::{
    ScrollView, ScrollConfig, ScrollDirection, ScrollState,
    ScrollBarTrack, ScrollBarThumb, ScrollIndicator, IndicatorPosition
};
pub use plugin::ScrollViewPlugin;
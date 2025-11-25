//! ScrollView module - Gateway only

// Private submodules
mod builder;
mod types;
mod plugin;
mod systems;

// Public exports
pub use builder::{ScrollViewBuilder, scroll_view, ScrollbarThumb};
pub use types::{
    ScrollView, ScrollConfig, ScrollDirection, ScrollbarVisibility,
    ScrollbarState, KineticScrollState, DragScrollTarget,
    ScrollIndicator, IndicatorPosition
};
pub use plugin::ScrollViewPlugin;

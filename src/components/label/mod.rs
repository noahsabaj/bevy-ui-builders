//! Label module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod types;

// Public exports only
pub use builder::{
    LabelBuilder, label, heading, title, secondary_text,
    error_label, success_label, warning_label,
};
pub use types::{Label, LabelSize};

// Deprecated re-export for backwards compatibility
#[allow(deprecated)]
pub use types::LabelStyle;
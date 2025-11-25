//! CheckboxBuilder for creating interactive checkboxes

mod builder;
mod plugin;
mod systems;
mod types;

// Public exports
pub use builder::CheckboxBuilder;
pub use plugin::CheckboxPlugin;
pub use types::{Checkbox, CheckboxState, CheckboxStyle, CheckboxStyleComponent, CheckboxCheckmark};

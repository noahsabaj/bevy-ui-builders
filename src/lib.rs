//! # Bevy UI Builders
//!
//! The UI builders Bevy should have shipped with - battle-tested in production.
//!
//! ## Example
//!
//! ```ignore
//! use bevy_ui_builders::*;
//!
//! ButtonBuilder::new("Click Me")
//!     .style(ButtonStyle::Primary)
//!     .build(commands);
//! ```

#![warn(missing_docs)]

// Re-export Bevy UI prelude for convenience
pub use bevy::prelude::*;

// Core modules
mod styles;
mod systems;
mod utils;

// Individual builder imports
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "form")]
pub mod form;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "text_input")]
pub mod text_input;
#[cfg(feature = "progress")]
pub mod progress;
#[cfg(feature = "label")]
pub mod label;
#[cfg(feature = "panel")]
pub mod panel;
#[cfg(feature = "separator")]
pub mod separator;

// Future modules (not implemented yet)
// #[cfg(feature = "tooltip")]
// pub mod tooltip;

// Public exports
pub use styles::{ButtonStyle, ButtonSize, colors, dimensions};
pub use systems::cleanup::{despawn_entities, despawn_ui_entities};
pub use systems::hover::HoverPlugin;

// Builder exports based on features
#[cfg(feature = "button")]
pub use button::{
    ButtonBuilder, StyledButton,
    primary_button, secondary_button, success_button, danger_button, ghost_button,
};

#[cfg(feature = "slider")]
pub use slider::{SliderBuilder, Slider, SliderHandle, SliderTrack, ValueFormat};

#[cfg(feature = "form")]
pub use form::{FormBuilder, FieldType, ValidationRule};

#[cfg(feature = "dialog")]
pub use dialog::{DialogBuilder, DialogButtonEvent, DialogType, DialogOverlay};

#[cfg(feature = "text_input")]
pub use text_input::{TextInputBuilder, InputFilter, InputTransform, FocusGroupId, text_input};

#[cfg(feature = "progress")]
pub use progress::{ProgressBarBuilder, ProgressBar, ProgressBarStyle, progress};

#[cfg(feature = "label")]
pub use label::{LabelBuilder, Label, LabelStyle, label};

#[cfg(feature = "panel")]
pub use panel::{PanelBuilder, Panel, PanelStyle, panel};

#[cfg(feature = "separator")]
pub use separator::{SeparatorBuilder, Separator, SeparatorStyle, Orientation, separator};

/// Prelude module for convenient imports
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        despawn_ui_entities, despawn_entities,
    };

    #[cfg(feature = "button")]
    pub use crate::{ButtonBuilder, ButtonStyle, ButtonSize};

    #[cfg(feature = "slider")]
    pub use crate::{SliderBuilder, ValueFormat};

    #[cfg(feature = "form")]
    pub use crate::{FormBuilder, FieldType, ValidationRule};

    #[cfg(feature = "dialog")]
    pub use crate::{DialogBuilder, DialogType};

    #[cfg(feature = "text_input")]
    pub use crate::{TextInputBuilder, InputFilter};

    #[cfg(feature = "progress")]
    pub use crate::{ProgressBarBuilder, ProgressBarStyle};

    #[cfg(feature = "label")]
    pub use crate::{LabelBuilder, LabelStyle};

    #[cfg(feature = "panel")]
    pub use crate::{PanelBuilder, PanelStyle};

    #[cfg(feature = "separator")]
    pub use crate::{SeparatorBuilder, Orientation};
}

/// Main plugin that adds all UI builder systems
pub struct UiBuilderPlugin;

impl Plugin for UiBuilderPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "button")]
        app.add_plugins(button::ButtonPlugin);

        #[cfg(feature = "slider")]
        app.add_plugins(slider::SliderPlugin);

        #[cfg(feature = "dialog")]
        app.add_plugins(dialog::DialogPlugin);

        #[cfg(feature = "text_input")]
        app.add_plugins(text_input::TextInputPlugin);

        #[cfg(feature = "progress")]
        app.add_plugins(progress::ProgressBarPlugin);

        // Add hover systems
        app.add_plugins(HoverPlugin);
    }
}
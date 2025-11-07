//! # Bevy UI Builders
//!
//! Declarative UI builders for Bevy.
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
use bevy_plugin_builder::define_plugin;

// Core modules
mod styles;
mod systems;
mod utils;
pub mod relationships;
pub mod validation;

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
#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "number_input")]
pub mod number_input;
#[cfg(feature = "dropdown")]
pub mod dropdown;

// ScrollView module (always available - core functionality)
pub mod scroll_view;

// Future modules (not implemented yet)
// #[cfg(feature = "tooltip")]
// pub mod tooltip;

// Public exports
pub use styles::{ButtonStyle, ButtonSize, colors, dimensions};
pub use systems::cleanup::{despawn_entities, despawn_ui_entities};
pub use systems::hover::HoverPlugin;
pub use scroll_view::{ScrollViewBuilder, ScrollView, ScrollConfig, ScrollDirection, ScrollViewPlugin, scroll_view};
pub use relationships::{
    BelongsToDialog, DialogElements,
    SliderPart, SliderParts,
    BelongsToForm, FormFields,
    InButtonGroup, ButtonGroupMembers,
    PanelContent, PanelContents,
    TextInputPart, TextInputParts,
    ProgressBarPart, ProgressBarParts,
    BelongsToDropdown, DropdownElements,
    UIRelationshipsPlugin,
};
pub use validation::{Validated, ValidationState, ValidationPlugin};

// Builder exports based on features
#[cfg(feature = "button")]
pub use button::{
    ButtonBuilder, StyledButton,
    primary_button, secondary_button, success_button, danger_button, ghost_button,
};

#[cfg(feature = "slider")]
pub use slider::{SliderBuilder, SliderBuilderWithMarker, Slider, SliderHandle, SliderTrack, ValueFormat};

#[cfg(feature = "form")]
pub use form::{FormBuilder, FieldType, ValidationRule};

#[cfg(feature = "dialog")]
pub use dialog::{
    DialogBuilder, DialogButtonEvent, DialogType, DialogOverlay, DialogButtonMarker,
    // Standard button markers for dialog buttons
    ConfirmButton, CancelButton, SaveButton, DiscardButton,
    OkButton, YesButton, NoButton,
    // Dialog type markers
    ExitConfirmationDialog, UnsavedChangesDialog, ResolutionDialog,
    ErrorDialog, InfoDialog, WarningDialog, SuccessDialog
};

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

#[cfg(feature = "checkbox")]
pub use checkbox::{CheckboxBuilder, Checkbox, CheckboxState, CheckboxStyle};

#[cfg(feature = "number_input")]
pub use number_input::{NumberInputBuilder, NumberInput, NumberInputConfig};

#[cfg(feature = "dropdown")]
pub use dropdown::{DropdownBuilder, Dropdown, DropdownState, DropdownData};

/// Prelude module for convenient imports
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        despawn_ui_entities, despawn_entities,
    };

    // Validation
    pub use crate::{Validated, ValidationState, ValidationRule};

    #[cfg(feature = "button")]
    pub use crate::{ButtonBuilder, ButtonStyle, ButtonSize};

    #[cfg(feature = "slider")]
    pub use crate::{SliderBuilder, ValueFormat};

    #[cfg(feature = "form")]
    pub use crate::{FormBuilder, FieldType};

    #[cfg(feature = "dialog")]
    pub use crate::{
        DialogBuilder, DialogType, DialogButtonMarker,
        // Standard button markers
        ConfirmButton, CancelButton, SaveButton, DiscardButton,
        OkButton, YesButton, NoButton,
    };

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

    #[cfg(feature = "checkbox")]
    pub use crate::{CheckboxBuilder, CheckboxState, CheckboxStyle};

    #[cfg(feature = "number_input")]
    pub use crate::{NumberInputBuilder};

    #[cfg(feature = "dropdown")]
    pub use crate::{DropdownBuilder, DropdownState};
}

define_plugin!(UiBuilderPlugin {
    plugins: [HoverPlugin, UIRelationshipsPlugin, ScrollViewPlugin, ValidationPlugin],
    custom_init: |app: &mut App| {
        // Bevy 0.17 requires picking plugins for Interaction component updates
        // Only add if not already present (DefaultPlugins includes them)
        if !app.is_plugin_added::<bevy::picking::input::PointerInputPlugin>() {
            app.add_plugins(bevy::picking::DefaultPickingPlugins);
        }
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

        #[cfg(feature = "checkbox")]
        app.add_plugins(checkbox::CheckboxPlugin);

        #[cfg(feature = "dropdown")]
        app.add_plugins(dropdown::DropdownPlugin);
    }
});
//! Dialog module - Gateway only

// Private submodules - no implementation here!
mod builder;
mod plugin;
mod systems;
mod types;

// Public exports only
pub use builder::{DialogBuilder, presets};
pub use plugin::DialogPlugin;
pub use systems::DialogButtonEvent;
pub use types::{
    DialogOverlay,
    DialogType,
    DialogContainer,
    DialogTitle,
    DialogBody,
    DialogButtonRow,
    DialogButton,
    DialogButtonMarker,
    // Dialog type markers
    ExitConfirmationDialog,
    UnsavedChangesDialog,
    ResolutionDialog,
    ErrorDialog,
    InfoDialog,
    WarningDialog,
    SuccessDialog,
    // Button markers
    ConfirmButton,
    CancelButton,
    SaveButton,
    DiscardButton,
    OkButton,
    YesButton,
    NoButton,
};
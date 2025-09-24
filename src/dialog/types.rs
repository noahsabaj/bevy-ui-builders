//! Dialog component types and markers

use bevy::prelude::*;

/// Component for dialog overlays
#[derive(Component, Debug, Clone)]
pub struct DialogOverlay {
    pub dialog_type: DialogType,
    pub dismissible: bool,
}

/// Types of dialogs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// Exit confirmation dialog
    ExitConfirmation,
    /// Unsaved changes warning
    UnsavedChanges,
    /// Resolution change confirmation
    Resolution,
    /// Error message
    Error,
    /// Information message
    Info,
    /// Warning message
    Warning,
    /// Success message
    Success,
    /// Custom dialog
    Custom,
}

/// Component for dialog containers
#[derive(Component, Debug)]
pub struct DialogContainer {
    pub dialog_type: DialogType,
}

/// Component for dialog title text
#[derive(Component)]
pub struct DialogTitle;

/// Component for dialog body text
#[derive(Component)]
pub struct DialogBody;

/// Component for dialog button row
#[derive(Component)]
pub struct DialogButtonRow;

/// Marker for exit confirmation dialog
#[derive(Component)]
pub struct ExitConfirmationDialog;

/// Marker for unsaved changes dialog
#[derive(Component)]
pub struct UnsavedChangesDialog;

/// Marker for resolution dialog
#[derive(Component)]
pub struct ResolutionDialog;

/// Marker for error dialog
#[derive(Component)]
pub struct ErrorDialog;

/// Marker for info dialog
#[derive(Component)]
pub struct InfoDialog;

/// Marker for warning dialog
#[derive(Component)]
pub struct WarningDialog;

/// Marker for success dialog
#[derive(Component)]
pub struct SuccessDialog;

/// Button markers for dialog actions
#[derive(Component)]
pub struct ConfirmButton;

#[derive(Component)]
pub struct CancelButton;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct DiscardButton;

#[derive(Component)]
pub struct OkButton;

#[derive(Component)]
pub struct YesButton;

#[derive(Component)]
pub struct NoButton;

/// Button configuration for dialogs
pub struct DialogButton {
    pub text: String,
    pub style: crate::ButtonStyle,
    pub marker: DialogButtonMarker,
}

/// Marker types for dialog buttons
pub enum DialogButtonMarker {
    Confirm,
    Cancel,
    Save,
    Discard,
    Ok,
    Yes,
    No,
    Custom(String),
}
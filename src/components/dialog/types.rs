//! Dialog component types and markers

use bevy::prelude::*;

use crate::theme::UiTheme;

/// Component for dialog overlays
#[derive(Component, Debug, Clone)]
pub struct DialogOverlay {
    /// The type of dialog being displayed
    pub dialog_type: DialogType,
    /// Whether the dialog can be dismissed by clicking outside
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
    /// The type of dialog this container belongs to
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

/// Marker for cancel button
#[derive(Component)]
pub struct CancelButton;

/// Marker for save button
#[derive(Component)]
pub struct SaveButton;

/// Marker for discard button
#[derive(Component)]
pub struct DiscardButton;

/// Marker for OK button
#[derive(Component)]
pub struct OkButton;

/// Marker for Yes button
#[derive(Component)]
pub struct YesButton;

/// Marker for No button
#[derive(Component)]
pub struct NoButton;

/// Button configuration for dialogs
pub struct DialogButton {
    /// Text to display on the button
    pub text: String,
    /// Visual style of the button
    pub style: crate::ButtonStyle,
    /// Marker to identify the button's action
    pub marker: DialogButtonMarker,
}

/// Marker types for dialog buttons
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DialogButtonMarker {
    /// Confirm action
    Confirm,
    /// Cancel action
    Cancel,
    /// Save action
    Save,
    /// Discard action
    Discard,
    /// OK action
    Ok,
    /// Yes action
    Yes,
    /// No action
    No,
    /// Custom action with identifier
    Custom(String),
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const OVERLAY_BACKDROP: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
    pub const BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.14);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
}

/// Resolved dialog colors from theme
#[derive(Clone)]
pub struct DialogColors {
    /// Overlay backdrop color
    pub overlay: Color,
    /// Container background color
    pub background: Color,
    /// Border color
    pub border: Color,
    /// Title text color
    pub text_title: Color,
    /// Body text color
    pub text_body: Color,
}

impl DialogColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        Self {
            overlay: theme.colors.overlay,
            background: theme.colors.surface.secondary,
            border: theme.colors.border.default,
            text_title: theme.colors.text.primary,
            text_body: theme.colors.text.secondary,
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            overlay: defaults::OVERLAY_BACKDROP,
            background: defaults::BACKGROUND,
            border: defaults::BORDER,
            text_title: defaults::TEXT_PRIMARY,
            text_body: defaults::TEXT_SECONDARY,
        }
    }
}
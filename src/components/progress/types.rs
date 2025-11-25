//! Progress bar component types and markers

use bevy::prelude::*;

use crate::theme::{UiTheme, SemanticVariant};

/// Component for progress bars
#[derive(Component, Debug)]
pub struct ProgressBar {
    /// Current progress value (0.0 to 1.0)
    pub value: f32,
    /// Visual style of the progress bar
    pub style: ProgressBarStyle,
    /// Whether the progress bar is animated
    pub animated: bool,
}

/// Progress bar style variants (controls size/height)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarStyle {
    /// Standard progress bar (default)
    #[default]
    Default,
    /// Thinner bar
    Thin,
    /// Thicker bar
    Thick,
    /// Segmented appearance
    Segmented,
}

impl ProgressBarStyle {
    /// Get the height associated with this style
    pub fn height(&self) -> f32 {
        match self {
            ProgressBarStyle::Thin => 4.0,
            ProgressBarStyle::Thick => 12.0,
            _ => 8.0,
        }
    }

    /// Get the track (background) color from theme
    pub fn track_color_from_theme(&self, theme: &UiTheme) -> Color {
        theme.colors.surface.secondary
    }

    /// Get the fill (foreground) color from theme based on variant
    pub fn fill_color_from_theme(&self, theme: &UiTheme, variant: SemanticVariant) -> Color {
        match variant {
            SemanticVariant::Primary => theme.colors.primary.base,
            SemanticVariant::Secondary => theme.colors.secondary.base,
            SemanticVariant::Success => theme.colors.success.base,
            SemanticVariant::Warning => theme.colors.warning.base,
            SemanticVariant::Danger => theme.colors.danger.base,
            SemanticVariant::Ghost => theme.colors.text.secondary,
        }
    }

    /// Get the label text color from theme
    pub fn label_color_from_theme(&self, theme: &UiTheme) -> Color {
        theme.colors.text.muted
    }
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const TRACK_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const FILL_COLOR: Color = Color::srgb(0.25, 0.46, 0.86); // Primary blue
    pub const FILL_SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);
    pub const FILL_WARNING: Color = Color::srgb(0.96, 0.76, 0.05);
    pub const FILL_DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
    pub const LABEL_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
}

impl ProgressBarStyle {
    /// Get the default track color (no theme)
    pub(crate) fn default_track_color(&self) -> Color {
        defaults::TRACK_COLOR
    }

    /// Get the default fill color based on variant (no theme)
    pub(crate) fn default_fill_color(&self, variant: SemanticVariant) -> Color {
        match variant {
            SemanticVariant::Primary => defaults::FILL_COLOR,
            SemanticVariant::Secondary => defaults::FILL_COLOR,
            SemanticVariant::Success => defaults::FILL_SUCCESS,
            SemanticVariant::Warning => defaults::FILL_WARNING,
            SemanticVariant::Danger => defaults::FILL_DANGER,
            SemanticVariant::Ghost => defaults::LABEL_COLOR,
        }
    }

    /// Get the default label color (no theme)
    pub(crate) fn default_label_color(&self) -> Color {
        defaults::LABEL_COLOR
    }
}

/// Marker component for the fill portion of a progress bar
#[derive(Component)]
pub struct ProgressBarFill;

/// Marker component for the track/background of a progress bar
#[derive(Component)]
pub struct ProgressBarTrack;

/// Marker component for the progress bar label
#[derive(Component)]
pub struct ProgressBarLabel;
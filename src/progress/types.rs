//! Progress bar component types and markers

use bevy::prelude::*;

/// Component for progress bars
#[derive(Component, Debug)]
pub struct ProgressBar {
    pub value: f32, // 0.0 to 1.0
    pub style: ProgressBarStyle,
    pub animated: bool,
}

/// Progress bar style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarStyle {
    #[default]
    Default,   // Standard progress bar
    Thin,      // Thinner bar
    Thick,     // Thicker bar
    Segmented, // Segmented appearance
}

impl ProgressBarStyle {
    pub fn height(&self) -> f32 {
        match self {
            ProgressBarStyle::Thin => 4.0,
            ProgressBarStyle::Thick => 12.0,
            _ => 8.0,
        }
    }

    pub fn track_color(&self) -> Color {
        crate::colors::BACKGROUND_DARK
    }

    pub fn fill_color(&self) -> Color {
        crate::colors::PRIMARY
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
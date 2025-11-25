//! Color constants and utilities for consistent theming
//!
//! # Deprecation Notice
//!
//! This module is deprecated. Use the theme system instead:
//!
//! ```ignore
//! // Instead of using colors::PRIMARY directly:
//! use bevy_ui_builders::theme::UiTheme;
//!
//! fn setup(theme: Res<UiTheme>) {
//!     // Access colors through the theme
//!     let primary = theme.colors.primary.base;
//!     let text = theme.colors.text.primary;
//! }
//!
//! // Builders now support .themed() for automatic theme integration:
//! ButtonBuilder::new("Click me")
//!     .themed(&theme)  // Uses theme colors
//!     .build(parent);
//! ```
//!
//! These constants are kept for backwards compatibility only.

#![allow(missing_docs)]

use bevy::prelude::*;

// Primary colors
pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);          // Blue
pub const PRIMARY_HOVER: Color = Color::srgb(0.35, 0.56, 0.96);
pub const PRIMARY_PRESSED: Color = Color::srgb(0.15, 0.36, 0.76);

pub const SECONDARY: Color = Color::srgb(0.25, 0.25, 0.25);        // Gray
pub const SECONDARY_HOVER: Color = Color::srgb(0.35, 0.35, 0.35);
pub const SECONDARY_PRESSED: Color = Color::srgb(0.15, 0.15, 0.15);

pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);          // Red
pub const DANGER_HOVER: Color = Color::srgb(0.96, 0.35, 0.35);
pub const DANGER_PRESSED: Color = Color::srgb(0.76, 0.15, 0.15);

pub const SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);         // Green
pub const SUCCESS_HOVER: Color = Color::srgb(0.35, 0.86, 0.35);
pub const SUCCESS_PRESSED: Color = Color::srgb(0.15, 0.66, 0.15);

pub const WARNING: Color = Color::srgb(0.96, 0.76, 0.05);         // Yellow
pub const WARNING_HOVER: Color = Color::srgb(1.0, 0.86, 0.15);
pub const WARNING_PRESSED: Color = Color::srgb(0.86, 0.66, 0.0);

pub const GHOST_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.20);
pub const GHOST_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.35);

// Dark variants for hover states
pub const PRIMARY_DARK: Color = Color::srgb(0.15, 0.36, 0.76);
pub const SECONDARY_DARK: Color = Color::srgb(0.15, 0.15, 0.15);
pub const SUCCESS_DARK: Color = Color::srgb(0.15, 0.66, 0.15);
pub const DANGER_DARK: Color = Color::srgb(0.76, 0.15, 0.15);

// Text colors on themed backgrounds
pub const TEXT_ON_PRIMARY: Color = Color::WHITE;
pub const TEXT_ON_SECONDARY: Color = Color::WHITE;
pub const TEXT_ON_SUCCESS: Color = Color::WHITE;
pub const TEXT_ON_DANGER: Color = Color::WHITE;

// Background colors
pub const BACKGROUND_PRIMARY: Color = Color::srgb(0.05, 0.05, 0.07);
pub const BACKGROUND_SECONDARY: Color = Color::srgb(0.08, 0.08, 0.1);
pub const BACKGROUND_TERTIARY: Color = Color::srgb(0.12, 0.12, 0.14);
pub const BACKGROUND_DARK: Color = Color::srgb(0.02, 0.02, 0.03);
pub const BACKGROUND_MEDIUM: Color = Color::srgb(0.1, 0.1, 0.12);
pub const BACKGROUND_LIGHT: Color = Color::srgb(0.15, 0.15, 0.17);

// Text colors
pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
pub const TEXT_DISABLED: Color = Color::srgb(0.4, 0.4, 0.4);
pub const TEXT_TITLE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);

// Border colors
pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
pub const BORDER_FOCUS: Color = PRIMARY;
pub const BORDER_ERROR: Color = DANGER;
pub const BORDER_LIGHT: Color = Color::srgb(0.5, 0.5, 0.5);

// Overlay colors
pub const OVERLAY_BACKDROP: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
pub const OVERLAY_LIGHT: Color = Color::srgba(1.0, 1.0, 1.0, 0.1);

/// Extension trait for Color to provide utility methods
pub trait ColorExt {
    /// Adjust the alpha channel
    fn with_alpha(&self, alpha: f32) -> Color;
}

impl ColorExt for Color {
    fn with_alpha(&self, alpha: f32) -> Color {
        let linear = self.to_linear();
        Color::LinearRgba(LinearRgba {
            red: linear.red,
            green: linear.green,
            blue: linear.blue,
            alpha: alpha.clamp(0.0, 1.0),
        })
    }
}
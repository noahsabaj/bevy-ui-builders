//! Pre-built theme presets.

use bevy::prelude::*;

use super::types::{
    UiTheme, ThemeColors, ColorScale, SurfaceColors, TextColors, BorderColors, GhostColors,
};
use super::spacing::ThemeSpacing;
use super::typography::ThemeTypography;
use super::borders::ThemeBorders;
use super::animation::ThemeAnimation;
use super::components::ComponentStyles;

impl UiTheme {
    /// Create the default dark theme.
    ///
    /// This is a modern dark theme suitable for games and applications.
    pub fn dark() -> Self {
        Self {
            colors: ThemeColors {
                primary: ColorScale::new(
                    Color::srgb(0.25, 0.46, 0.86),  // Blue
                    Color::srgb(0.35, 0.56, 0.96),  // Lighter blue (hover)
                    Color::srgb(0.15, 0.36, 0.76),  // Darker blue (pressed)
                    Color::srgba(0.25, 0.46, 0.86, 0.5), // Faded (disabled)
                    Color::WHITE,                   // Text on primary
                ),
                secondary: ColorScale::new(
                    Color::srgb(0.25, 0.25, 0.25),  // Gray
                    Color::srgb(0.35, 0.35, 0.35),  // Lighter gray
                    Color::srgb(0.15, 0.15, 0.15),  // Darker gray
                    Color::srgba(0.25, 0.25, 0.25, 0.5),
                    Color::WHITE,
                ),
                success: ColorScale::new(
                    Color::srgb(0.25, 0.76, 0.25),  // Green
                    Color::srgb(0.35, 0.86, 0.35),
                    Color::srgb(0.15, 0.66, 0.15),
                    Color::srgba(0.25, 0.76, 0.25, 0.5),
                    Color::WHITE,
                ),
                warning: ColorScale::new(
                    Color::srgb(0.96, 0.76, 0.05),  // Yellow/Orange
                    Color::srgb(1.0, 0.86, 0.15),
                    Color::srgb(0.86, 0.66, 0.0),
                    Color::srgba(0.96, 0.76, 0.05, 0.5),
                    Color::BLACK,  // Dark text on yellow
                ),
                danger: ColorScale::new(
                    Color::srgb(0.86, 0.25, 0.25),  // Red
                    Color::srgb(0.96, 0.35, 0.35),
                    Color::srgb(0.76, 0.15, 0.15),
                    Color::srgba(0.86, 0.25, 0.25, 0.5),
                    Color::WHITE,
                ),
                ghost: GhostColors {
                    hover: Color::srgba(1.0, 1.0, 1.0, 0.20),
                    pressed: Color::srgba(1.0, 1.0, 1.0, 0.35),
                },
                surface: SurfaceColors {
                    background: Color::srgb(0.05, 0.05, 0.07),
                    primary: Color::srgb(0.08, 0.08, 0.1),
                    secondary: Color::srgb(0.12, 0.12, 0.14),
                    tertiary: Color::srgb(0.15, 0.15, 0.17),
                    dark: Color::srgb(0.02, 0.02, 0.03),
                    medium: Color::srgb(0.1, 0.1, 0.12),
                    light: Color::srgb(0.15, 0.15, 0.17),
                },
                text: TextColors {
                    primary: Color::srgb(0.95, 0.95, 0.95),
                    secondary: Color::srgb(0.7, 0.7, 0.7),
                    disabled: Color::srgb(0.4, 0.4, 0.4),
                    title: Color::WHITE,
                    muted: Color::srgb(0.5, 0.5, 0.5),
                    link: Color::srgb(0.35, 0.56, 0.96),
                },
                border: BorderColors {
                    default: Color::srgb(0.3, 0.3, 0.3),
                    focus: Color::srgb(0.25, 0.46, 0.86),  // Primary color
                    error: Color::srgb(0.86, 0.25, 0.25),  // Danger color
                    success: Color::srgb(0.25, 0.76, 0.25), // Success color
                    light: Color::srgb(0.5, 0.5, 0.5),
                },
                overlay: Color::srgba(0.0, 0.0, 0.0, 0.6),
            },
            typography: ThemeTypography::default(),
            spacing: ThemeSpacing::default(),
            borders: ThemeBorders::default(),
            animation: ThemeAnimation::default(),
            components: ComponentStyles::default(),
        }
    }

    /// Create a light theme.
    ///
    /// A clean light theme suitable for tools and applications.
    pub fn light() -> Self {
        Self {
            colors: ThemeColors {
                primary: ColorScale::new(
                    Color::srgb(0.20, 0.40, 0.80),  // Blue
                    Color::srgb(0.15, 0.35, 0.75),  // Darker on hover (inverted from dark theme)
                    Color::srgb(0.10, 0.30, 0.70),  // Even darker on press
                    Color::srgba(0.20, 0.40, 0.80, 0.5),
                    Color::WHITE,
                ),
                secondary: ColorScale::new(
                    Color::srgb(0.6, 0.6, 0.6),    // Gray
                    Color::srgb(0.5, 0.5, 0.5),
                    Color::srgb(0.4, 0.4, 0.4),
                    Color::srgba(0.6, 0.6, 0.6, 0.5),
                    Color::WHITE,
                ),
                success: ColorScale::new(
                    Color::srgb(0.20, 0.65, 0.20),
                    Color::srgb(0.15, 0.55, 0.15),
                    Color::srgb(0.10, 0.45, 0.10),
                    Color::srgba(0.20, 0.65, 0.20, 0.5),
                    Color::WHITE,
                ),
                warning: ColorScale::new(
                    Color::srgb(0.90, 0.70, 0.0),
                    Color::srgb(0.80, 0.60, 0.0),
                    Color::srgb(0.70, 0.50, 0.0),
                    Color::srgba(0.90, 0.70, 0.0, 0.5),
                    Color::BLACK,
                ),
                danger: ColorScale::new(
                    Color::srgb(0.80, 0.20, 0.20),
                    Color::srgb(0.70, 0.15, 0.15),
                    Color::srgb(0.60, 0.10, 0.10),
                    Color::srgba(0.80, 0.20, 0.20, 0.5),
                    Color::WHITE,
                ),
                ghost: GhostColors {
                    hover: Color::srgba(0.0, 0.0, 0.0, 0.18),
                    pressed: Color::srgba(0.0, 0.0, 0.0, 0.30),
                },
                surface: SurfaceColors {
                    background: Color::srgb(0.96, 0.96, 0.98),
                    primary: Color::WHITE,
                    secondary: Color::srgb(0.98, 0.98, 0.99),
                    tertiary: Color::srgb(0.94, 0.94, 0.96),
                    dark: Color::srgb(0.90, 0.90, 0.92),
                    medium: Color::srgb(0.92, 0.92, 0.94),
                    light: Color::srgb(0.96, 0.96, 0.98),
                },
                text: TextColors {
                    primary: Color::srgb(0.1, 0.1, 0.1),
                    secondary: Color::srgb(0.4, 0.4, 0.4),
                    disabled: Color::srgb(0.6, 0.6, 0.6),
                    title: Color::BLACK,
                    muted: Color::srgb(0.5, 0.5, 0.5),
                    link: Color::srgb(0.20, 0.40, 0.80),
                },
                border: BorderColors {
                    default: Color::srgb(0.8, 0.8, 0.8),
                    focus: Color::srgb(0.20, 0.40, 0.80),
                    error: Color::srgb(0.80, 0.20, 0.20),
                    success: Color::srgb(0.20, 0.65, 0.20),
                    light: Color::srgb(0.85, 0.85, 0.85),
                },
                overlay: Color::srgba(0.0, 0.0, 0.0, 0.4),
            },
            typography: ThemeTypography::default(),
            spacing: ThemeSpacing::default(),
            borders: ThemeBorders::default(),
            animation: ThemeAnimation::default(),
            components: ComponentStyles::default(),
        }
    }

    /// Create a high-contrast theme for accessibility.
    pub fn high_contrast() -> Self {
        Self {
            colors: ThemeColors {
                primary: ColorScale::new(
                    Color::srgb(0.0, 0.5, 1.0),    // Bright blue
                    Color::srgb(0.2, 0.6, 1.0),
                    Color::srgb(0.0, 0.4, 0.9),
                    Color::srgba(0.0, 0.5, 1.0, 0.5),
                    Color::WHITE,
                ),
                secondary: ColorScale::new(
                    Color::srgb(0.3, 0.3, 0.3),
                    Color::srgb(0.4, 0.4, 0.4),
                    Color::srgb(0.2, 0.2, 0.2),
                    Color::srgba(0.3, 0.3, 0.3, 0.5),
                    Color::WHITE,
                ),
                success: ColorScale::new(
                    Color::srgb(0.0, 0.8, 0.0),    // Bright green
                    Color::srgb(0.2, 0.9, 0.2),
                    Color::srgb(0.0, 0.7, 0.0),
                    Color::srgba(0.0, 0.8, 0.0, 0.5),
                    Color::BLACK,
                ),
                warning: ColorScale::new(
                    Color::srgb(1.0, 0.8, 0.0),    // Bright yellow
                    Color::srgb(1.0, 0.9, 0.2),
                    Color::srgb(0.9, 0.7, 0.0),
                    Color::srgba(1.0, 0.8, 0.0, 0.5),
                    Color::BLACK,
                ),
                danger: ColorScale::new(
                    Color::srgb(1.0, 0.0, 0.0),    // Bright red
                    Color::srgb(1.0, 0.2, 0.2),
                    Color::srgb(0.9, 0.0, 0.0),
                    Color::srgba(1.0, 0.0, 0.0, 0.5),
                    Color::WHITE,
                ),
                ghost: GhostColors::default(),
                surface: SurfaceColors {
                    background: Color::BLACK,
                    primary: Color::srgb(0.05, 0.05, 0.05),
                    secondary: Color::srgb(0.1, 0.1, 0.1),
                    tertiary: Color::srgb(0.15, 0.15, 0.15),
                    dark: Color::BLACK,
                    medium: Color::srgb(0.08, 0.08, 0.08),
                    light: Color::srgb(0.12, 0.12, 0.12),
                },
                text: TextColors {
                    primary: Color::WHITE,
                    secondary: Color::srgb(0.9, 0.9, 0.9),
                    disabled: Color::srgb(0.5, 0.5, 0.5),
                    title: Color::WHITE,
                    muted: Color::srgb(0.7, 0.7, 0.7),
                    link: Color::srgb(0.3, 0.7, 1.0),
                },
                border: BorderColors {
                    default: Color::WHITE,
                    focus: Color::srgb(0.0, 0.5, 1.0),
                    error: Color::srgb(1.0, 0.0, 0.0),
                    success: Color::srgb(0.0, 0.8, 0.0),
                    light: Color::srgb(0.8, 0.8, 0.8),
                },
                overlay: Color::srgba(0.0, 0.0, 0.0, 0.8),
            },
            typography: ThemeTypography::default(),
            spacing: ThemeSpacing::default(),
            borders: ThemeBorders::default(),
            animation: ThemeAnimation::default(),
            components: ComponentStyles::default(),
        }
    }
}

impl Default for UiTheme {
    fn default() -> Self {
        Self::dark()
    }
}

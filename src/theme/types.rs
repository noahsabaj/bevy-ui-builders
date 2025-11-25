//! Core theme types for the UI system.
//!
//! This module defines the fundamental types for theming:
//! - `UiTheme`: The main theme resource containing all styling information
//! - `ThemeColors`: Semantic color definitions
//! - `ColorScale`: Colors with interaction states (base, hover, pressed, disabled)

use bevy::prelude::*;

use super::borders::ThemeBorders;
use super::components::ComponentStyles;
use super::spacing::ThemeSpacing;
use super::typography::ThemeTypography;
use super::animation::ThemeAnimation;

/// The main UI theme resource.
///
/// This resource contains all styling information for the UI system.
/// Change this resource to update the appearance of all UI components.
///
/// # Example
///
/// ```ignore
/// // Use the default dark theme
/// app.insert_resource(UiTheme::dark());
///
/// // Or customize it
/// app.insert_resource(
///     UiTheme::dark()
///         .with_primary(Color::srgb(0.8, 0.2, 0.5))
/// );
/// ```
#[derive(Resource, Clone, Debug)]
pub struct UiTheme {
    /// Semantic color palette
    pub colors: ThemeColors,
    /// Typography settings
    pub typography: ThemeTypography,
    /// Spacing scale
    pub spacing: ThemeSpacing,
    /// Border settings
    pub borders: ThemeBorders,
    /// Animation settings
    pub animation: ThemeAnimation,
    /// Component-specific style overrides
    pub components: ComponentStyles,
}

impl UiTheme {
    /// Create a new theme with custom colors (uses defaults for everything else)
    pub fn new(colors: ThemeColors) -> Self {
        Self {
            colors,
            typography: ThemeTypography::default(),
            spacing: ThemeSpacing::default(),
            borders: ThemeBorders::default(),
            animation: ThemeAnimation::default(),
            components: ComponentStyles::default(),
        }
    }

    /// Set the primary color and auto-derive its interaction states
    pub fn with_primary(mut self, color: Color) -> Self {
        self.colors.primary = ColorScale::from_base(color);
        self
    }

    /// Set the secondary color and auto-derive its interaction states
    pub fn with_secondary(mut self, color: Color) -> Self {
        self.colors.secondary = ColorScale::from_base(color);
        self
    }

    /// Set the success color and auto-derive its interaction states
    pub fn with_success(mut self, color: Color) -> Self {
        self.colors.success = ColorScale::from_base(color);
        self
    }

    /// Set the warning color and auto-derive its interaction states
    pub fn with_warning(mut self, color: Color) -> Self {
        self.colors.warning = ColorScale::from_base(color);
        self
    }

    /// Set the danger color and auto-derive its interaction states
    pub fn with_danger(mut self, color: Color) -> Self {
        self.colors.danger = ColorScale::from_base(color);
        self
    }

    /// Scale all typography by a factor
    pub fn with_font_scale(mut self, scale: f32) -> Self {
        self.typography = self.typography.scaled(scale);
        self
    }

    /// Scale all spacing by a factor
    pub fn with_spacing_scale(mut self, scale: f32) -> Self {
        self.spacing = self.spacing.scaled(scale);
        self
    }
}

/// Semantic color palette for the theme.
///
/// Colors are organized by intent (primary, danger, success, etc.)
/// rather than by visual appearance (blue, red, green).
#[derive(Clone, Debug)]
pub struct ThemeColors {
    /// Primary brand color - used for main actions and focus
    pub primary: ColorScale,
    /// Secondary color - used for less prominent elements
    pub secondary: ColorScale,
    /// Success color - positive actions and states
    pub success: ColorScale,
    /// Warning color - cautionary actions and states
    pub warning: ColorScale,
    /// Danger color - destructive actions and errors
    pub danger: ColorScale,
    /// Ghost/transparent style colors
    pub ghost: GhostColors,
    /// Surface/background colors
    pub surface: SurfaceColors,
    /// Text colors
    pub text: TextColors,
    /// Border colors
    pub border: BorderColors,
    /// Overlay backdrop color
    pub overlay: Color,
}

/// Colors for ghost/transparent style elements
#[derive(Clone, Debug)]
pub struct GhostColors {
    /// Background on hover
    pub hover: Color,
    /// Background when pressed
    pub pressed: Color,
}

impl Default for GhostColors {
    fn default() -> Self {
        Self {
            hover: Color::srgba(1.0, 1.0, 1.0, 0.20),
            pressed: Color::srgba(1.0, 1.0, 1.0, 0.35),
        }
    }
}

/// A complete color scale with all interaction states.
///
/// No optional fields - every state must be defined for consistency.
#[derive(Clone, Debug)]
pub struct ColorScale {
    /// Base/default color
    pub base: Color,
    /// Color when hovered
    pub hover: Color,
    /// Color when pressed/active
    pub pressed: Color,
    /// Color when disabled
    pub disabled: Color,
    /// Contrasting text color for use on this background
    pub on_color: Color,
}

impl ColorScale {
    /// Create a color scale with all states explicitly defined
    pub fn new(
        base: Color,
        hover: Color,
        pressed: Color,
        disabled: Color,
        on_color: Color,
    ) -> Self {
        Self {
            base,
            hover,
            pressed,
            disabled,
            on_color,
        }
    }

    /// Auto-derive interaction states from a base color.
    ///
    /// - Hover: 10% lighter
    /// - Pressed: 10% darker
    /// - Disabled: 50% desaturated with reduced opacity
    /// - On-color: Auto-calculated for contrast (white or black)
    pub fn from_base(base: Color) -> Self {
        use bevy::color::Alpha;
        Self {
            base,
            hover: lighten(base, 0.1),
            pressed: darken(base, 0.1),
            disabled: desaturate(base, 0.5).with_alpha(0.5),
            on_color: contrast_color(base),
        }
    }

    /// Get colors as a tuple (base, text, border) for backwards compatibility
    pub fn as_tuple(&self) -> (Color, Color, Color) {
        (self.base, self.on_color, self.pressed)
    }
}

/// Surface/background colors for layering
#[derive(Clone, Debug)]
pub struct SurfaceColors {
    /// Main app background
    pub background: Color,
    /// Primary surface (cards, panels)
    pub primary: Color,
    /// Secondary/elevated surface
    pub secondary: Color,
    /// Tertiary/highest elevation
    pub tertiary: Color,
    /// Dark variant
    pub dark: Color,
    /// Medium variant
    pub medium: Color,
    /// Light variant
    pub light: Color,
}

/// Text colors for different contexts
#[derive(Clone, Debug)]
pub struct TextColors {
    /// Primary text color (highest contrast)
    pub primary: Color,
    /// Secondary text color (medium contrast)
    pub secondary: Color,
    /// Disabled text color (low contrast)
    pub disabled: Color,
    /// Title/heading text color
    pub title: Color,
    /// Muted/subtle text color
    pub muted: Color,
    /// Link text color
    pub link: Color,
}

/// Border colors for different states
#[derive(Clone, Debug)]
pub struct BorderColors {
    /// Default border color
    pub default: Color,
    /// Border color when focused
    pub focus: Color,
    /// Border color for errors
    pub error: Color,
    /// Border color for success
    pub success: Color,
    /// Light border variant
    pub light: Color,
}

// Color manipulation utilities

/// Lighten a color by a factor (0.0 to 1.0)
pub fn lighten(color: Color, factor: f32) -> Color {
    let linear = color.to_linear();
    Color::LinearRgba(LinearRgba {
        red: (linear.red + (1.0 - linear.red) * factor).clamp(0.0, 1.0),
        green: (linear.green + (1.0 - linear.green) * factor).clamp(0.0, 1.0),
        blue: (linear.blue + (1.0 - linear.blue) * factor).clamp(0.0, 1.0),
        alpha: linear.alpha,
    })
}

/// Darken a color by a factor (0.0 to 1.0)
pub fn darken(color: Color, factor: f32) -> Color {
    let linear = color.to_linear();
    Color::LinearRgba(LinearRgba {
        red: (linear.red * (1.0 - factor)).clamp(0.0, 1.0),
        green: (linear.green * (1.0 - factor)).clamp(0.0, 1.0),
        blue: (linear.blue * (1.0 - factor)).clamp(0.0, 1.0),
        alpha: linear.alpha,
    })
}

/// Desaturate a color by a factor (0.0 to 1.0)
pub fn desaturate(color: Color, factor: f32) -> Color {
    let linear = color.to_linear();
    let luminance = 0.299 * linear.red + 0.587 * linear.green + 0.114 * linear.blue;
    Color::LinearRgba(LinearRgba {
        red: linear.red + (luminance - linear.red) * factor,
        green: linear.green + (luminance - linear.green) * factor,
        blue: linear.blue + (luminance - linear.blue) * factor,
        alpha: linear.alpha,
    })
}

/// Calculate a contrasting color (white or black) for text on the given background
pub fn contrast_color(background: Color) -> Color {
    let linear = background.to_linear();
    // Calculate relative luminance using sRGB coefficients
    let luminance = 0.299 * linear.red + 0.587 * linear.green + 0.114 * linear.blue;

    // Use white text on dark backgrounds, black on light
    if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

// Note: Use bevy::color::Alpha trait for with_alpha() functionality

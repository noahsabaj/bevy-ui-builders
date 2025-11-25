//! Theme system for bevy-ui-builders.
//!
//! The theme system provides:
//! - `UiTheme`: A resource containing all styling information
//! - Semantic colors with interaction states
//! - Typography, spacing, and border configurations
//! - Pre-built theme presets (dark, light, high-contrast)
//!
//! # Quick Start
//!
//! ```ignore
//! use bevy_ui_builders::prelude::*;
//!
//! // Use the default dark theme (automatic with UiBuilderPlugin)
//! app.add_plugins(UiBuilderPlugin);
//!
//! // Or customize
//! app.insert_resource(
//!     UiTheme::dark()
//!         .with_primary(Color::srgb(0.8, 0.2, 0.5))
//! );
//! ```
//!
//! # Theme Structure
//!
//! The theme is organized into:
//! - **Colors**: Semantic colors (primary, danger, success, etc.) with interaction states
//! - **Typography**: Font sizes and line heights
//! - **Spacing**: Consistent spacing scale
//! - **Borders**: Border widths and radii
//! - **Animation**: Duration and easing presets
//! - **Components**: Component-specific style overrides

mod animation;
mod borders;
mod components;
mod plugin;
mod presets;
mod spacing;
mod typography;
mod types;
mod variant;

// Re-export all public types
pub use animation::{
    AnimationCategories, AnimationDurations, AnimationEasing, CategoryDefaults, EasingType,
    HoverDefaults, ThemeAnimation,
};
pub use borders::{BorderRadii, BorderWidths, ThemeBorders};
pub use components::{
    ButtonComponentStyle, CheckboxComponentStyle, ComponentStyles, DialogComponentStyle,
    DropdownComponentStyle, PanelComponentStyle, ProgressComponentStyle, SliderComponentStyle,
    TextInputComponentStyle,
};
pub use plugin::{detect_theme_changes, ThemeChanged, ThemePlugin};
pub use spacing::{ComponentSpacing, SpacingLevel, SpacingScale, ThemeSpacing};
pub use typography::{LineHeights, ThemeTypography, TypeScale};
pub use types::{
    contrast_color, darken, desaturate, lighten, BorderColors, ColorScale,
    GhostColors, SurfaceColors, TextColors, ThemeColors, UiTheme,
};
pub use variant::{
    resolve_colors, ColorScaleRef, ResolvedColors, SemanticVariant,
};

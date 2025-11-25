//! Semantic variants and color resolution for themed UI components.
//!
//! This module provides:
//! - `SemanticVariant`: Unified color variant for all components
//! - `ColorScaleRef`: Efficient reference to resolved colors
//! - `ResolvedColors`: Flat color structure ready for use in builders

use bevy::prelude::*;

use super::types::{ColorScale, UiTheme, lighten, darken};

/// Unified semantic variant for all themed UI components.
///
/// This enum represents the semantic meaning of a component's appearance,
/// not its specific colors. Colors are resolved from the active theme.
///
/// # Example
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// ButtonBuilder::new("Save")
///     .variant(SemanticVariant::Success)
///     .build(parent);
///
/// ButtonBuilder::new("Delete")
///     .variant(SemanticVariant::Danger)
///     .build(parent);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SemanticVariant {
    /// Primary brand color - main actions and focus states
    #[default]
    Primary,
    /// Secondary/neutral - less prominent elements
    Secondary,
    /// Success/positive - confirmations, completions
    Success,
    /// Warning/caution - important but not destructive
    Warning,
    /// Danger/destructive - errors, deletions, irreversible actions
    Danger,
    /// Ghost/transparent - minimal visual presence
    Ghost,
}

impl SemanticVariant {
    /// Resolve this variant to a color reference from the theme.
    ///
    /// Returns a `ColorScaleRef` which can be either a reference to
    /// an existing `ColorScale` or custom ghost colors.
    pub fn color_scale<'a>(&self, theme: &'a UiTheme) -> ColorScaleRef<'a> {
        match self {
            Self::Primary => ColorScaleRef::Scale(&theme.colors.primary),
            Self::Secondary => ColorScaleRef::Scale(&theme.colors.secondary),
            Self::Success => ColorScaleRef::Scale(&theme.colors.success),
            Self::Warning => ColorScaleRef::Scale(&theme.colors.warning),
            Self::Danger => ColorScaleRef::Scale(&theme.colors.danger),
            Self::Ghost => ColorScaleRef::Ghost {
                hover: theme.colors.ghost.hover,
                pressed: theme.colors.ghost.pressed,
                text: theme.colors.text.secondary,
                border: theme.colors.border.default,
            },
        }
    }

    /// Get all variants as an array (useful for iteration/UI)
    pub fn all() -> [Self; 6] {
        [
            Self::Primary,
            Self::Secondary,
            Self::Success,
            Self::Warning,
            Self::Danger,
            Self::Ghost,
        ]
    }
}

/// Reference to resolved colors, avoiding unnecessary clones.
///
/// This enum handles the special case of Ghost variants which don't
/// have a standard ColorScale but instead use transparent backgrounds.
#[derive(Debug, Clone)]
pub enum ColorScaleRef<'a> {
    /// Reference to a standard ColorScale
    Scale(&'a ColorScale),
    /// Ghost style with transparent background and custom hover/pressed
    Ghost {
        /// Background on hover (semi-transparent)
        hover: Color,
        /// Background when pressed (semi-transparent)
        pressed: Color,
        /// Text color
        text: Color,
        /// Border color
        border: Color,
    },
}

impl<'a> ColorScaleRef<'a> {
    /// Convert to ResolvedColors for use in builders
    pub fn resolve(&self) -> ResolvedColors {
        match self {
            Self::Scale(scale) => ResolvedColors::from_scale(scale),
            Self::Ghost { hover, pressed, text, border } => ResolvedColors {
                background: Color::NONE,
                background_hover: *hover,
                background_pressed: *pressed,
                background_disabled: Color::NONE,
                text: *text,
                border: *border,
            },
        }
    }
}

/// Flat structure of resolved colors ready for use in builders.
///
/// This struct contains all the colors needed to render a component
/// with proper interaction states.
#[derive(Clone, Debug)]
pub struct ResolvedColors {
    /// Background color (normal state)
    pub background: Color,
    /// Background color when hovered
    pub background_hover: Color,
    /// Background color when pressed
    pub background_pressed: Color,
    /// Background color when disabled
    pub background_disabled: Color,
    /// Text/foreground color
    pub text: Color,
    /// Border color
    pub border: Color,
}

impl ResolvedColors {
    /// Create from a ColorScale (standard path)
    pub fn from_scale(scale: &ColorScale) -> Self {
        Self {
            background: scale.base,
            background_hover: scale.hover,
            background_pressed: scale.pressed,
            background_disabled: scale.disabled,
            text: scale.on_color,
            border: scale.pressed, // Use pressed as default border color
        }
    }

    /// Apply optional color overrides
    ///
    /// Call this after creating from variant to apply any user-specified overrides.
    pub fn with_overrides(
        mut self,
        background: Option<Color>,
        text: Option<Color>,
        border: Option<Color>,
    ) -> Self {
        if let Some(bg) = background {
            self.background = bg;
            // Auto-derive hover/pressed from new background
            self.background_hover = lighten(bg, 0.1);
            self.background_pressed = darken(bg, 0.1);
        }
        if let Some(t) = text {
            self.text = t;
        }
        if let Some(b) = border {
            self.border = b;
        }
        self
    }

    /// Apply a complete ColorScale override
    ///
    /// This replaces all colors with those from the provided scale.
    pub fn with_scale_override(self, scale: Option<&ColorScale>) -> Self {
        match scale {
            Some(s) => Self::from_scale(s),
            None => self,
        }
    }
}

/// Helper function to resolve colors from variant with overrides.
///
/// This is the main entry point for builders to get themed colors.
///
/// # Arguments
///
/// * `variant` - The semantic variant to resolve
/// * `theme` - The active UI theme
/// * `override_background` - Optional background color override
/// * `override_text` - Optional text color override
/// * `override_border` - Optional border color override
/// * `override_scale` - Optional complete ColorScale override (takes precedence)
///
/// # Example
///
/// ```ignore
/// let colors = resolve_colors(
///     self.variant,
///     &theme,
///     self.override_background,
///     self.override_text,
///     self.override_border,
///     self.override_scale.as_ref(),
/// );
/// ```
pub fn resolve_colors(
    variant: SemanticVariant,
    theme: &UiTheme,
    override_background: Option<Color>,
    override_text: Option<Color>,
    override_border: Option<Color>,
    override_scale: Option<&ColorScale>,
) -> ResolvedColors {
    // Start with colors from variant
    let colors = variant.color_scale(theme).resolve();

    // Apply scale override first (takes full precedence)
    let colors = colors.with_scale_override(override_scale);

    // Then apply individual overrides
    colors.with_overrides(override_background, override_text, override_border)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_variant_default() {
        assert_eq!(SemanticVariant::default(), SemanticVariant::Primary);
    }

    #[test]
    fn test_all_variants() {
        let variants = SemanticVariant::all();
        assert_eq!(variants.len(), 6);
        assert!(variants.contains(&SemanticVariant::Primary));
        assert!(variants.contains(&SemanticVariant::Ghost));
    }
}

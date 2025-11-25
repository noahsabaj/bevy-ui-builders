//! Spacing scale for consistent layout.

use bevy::prelude::*;

/// Spacing scale for the theme.
///
/// Based on a unit system where all spacing is derived from a base unit.
#[derive(Clone, Debug)]
pub struct ThemeSpacing {
    /// Base unit (typically 4px)
    pub unit: f32,
    /// Spacing scale
    pub scale: SpacingScale,
    /// Component-specific spacing
    pub components: ComponentSpacing,
}

impl ThemeSpacing {
    /// Create spacing from a base unit
    pub fn from_unit(unit: f32) -> Self {
        Self {
            unit,
            scale: SpacingScale::from_unit(unit),
            components: ComponentSpacing::from_unit(unit),
        }
    }

    /// Scale all spacing values by a factor
    pub fn scaled(self, factor: f32) -> Self {
        Self::from_unit(self.unit * factor)
    }
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self::from_unit(4.0)
    }
}

/// Standard spacing scale values
#[derive(Clone, Debug)]
pub struct SpacingScale {
    /// No spacing (0)
    pub none: f32,
    /// Extra small (4px at unit=4)
    pub xs: f32,
    /// Small (8px at unit=4)
    pub sm: f32,
    /// Medium (16px at unit=4)
    pub md: f32,
    /// Large (24px at unit=4)
    pub lg: f32,
    /// Extra large (32px at unit=4)
    pub xl: f32,
    /// 2x extra large (48px at unit=4)
    pub xxl: f32,
}

impl SpacingScale {
    /// Create a spacing scale from a base unit
    pub fn from_unit(unit: f32) -> Self {
        Self {
            none: 0.0,
            xs: unit,
            sm: unit * 2.0,
            md: unit * 4.0,
            lg: unit * 6.0,
            xl: unit * 8.0,
            xxl: unit * 12.0,
        }
    }

    /// Get spacing as Val::Px for the given level
    pub fn as_val(&self, level: SpacingLevel) -> Val {
        Val::Px(match level {
            SpacingLevel::None => self.none,
            SpacingLevel::Xs => self.xs,
            SpacingLevel::Sm => self.sm,
            SpacingLevel::Md => self.md,
            SpacingLevel::Lg => self.lg,
            SpacingLevel::Xl => self.xl,
            SpacingLevel::Xxl => self.xxl,
        })
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self::from_unit(4.0)
    }
}

/// Named spacing levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingLevel {
    /// No spacing
    None,
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
    /// 2x extra large
    Xxl,
}

/// Component-specific spacing values
#[derive(Clone, Debug)]
pub struct ComponentSpacing {
    /// Padding for small elements
    pub padding_sm: f32,
    /// Padding for medium elements
    pub padding_md: f32,
    /// Padding for large elements
    pub padding_lg: f32,
    /// Default panel padding
    pub panel_padding: f32,
    /// Small margin
    pub margin_sm: f32,
    /// Medium margin
    pub margin_md: f32,
    /// Large margin
    pub margin_lg: f32,
    /// Separator margin
    pub separator_margin: f32,
    /// Gap between inline elements
    pub inline_gap: f32,
    /// Gap between stacked elements
    pub stack_gap: f32,
}

impl ComponentSpacing {
    /// Create component spacing from a base unit
    pub fn from_unit(unit: f32) -> Self {
        Self {
            padding_sm: unit * 2.0,       // 8px
            padding_md: unit * 3.0,       // 12px
            padding_lg: unit * 4.0,       // 16px
            panel_padding: unit * 4.0,    // 16px
            margin_sm: unit * 2.0,        // 8px
            margin_md: unit * 4.0,        // 16px
            margin_lg: unit * 6.0,        // 24px
            separator_margin: unit * 3.0, // 12px
            inline_gap: unit * 2.0,       // 8px
            stack_gap: unit * 4.0,        // 16px
        }
    }
}

impl Default for ComponentSpacing {
    fn default() -> Self {
        Self::from_unit(4.0)
    }
}

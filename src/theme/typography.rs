//! Typography settings for text rendering.

/// Typography settings for the theme.
#[derive(Clone, Debug)]
pub struct ThemeTypography {
    /// Type scale for font sizes
    pub scale: TypeScale,
    /// Line height multipliers
    pub line_height: LineHeights,
}

impl ThemeTypography {
    /// Create typography with a custom base size
    pub fn with_base(base: f32) -> Self {
        Self {
            scale: TypeScale::from_base(base),
            line_height: LineHeights::default(),
        }
    }

    /// Scale all typography by a factor
    pub fn scaled(self, factor: f32) -> Self {
        Self {
            scale: self.scale.scaled(factor),
            line_height: self.line_height,
        }
    }
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            scale: TypeScale::default(),
            line_height: LineHeights::default(),
        }
    }
}

/// Font size scale
#[derive(Clone, Debug)]
pub struct TypeScale {
    /// Extra small (10px)
    pub xs: f32,
    /// Small (12px)
    pub sm: f32,
    /// Base/body (14px)
    pub base: f32,
    /// Medium (16px)
    pub md: f32,
    /// Large (18px)
    pub lg: f32,
    /// Extra large (20px)
    pub xl: f32,
    /// 2x extra large (24px)
    pub xxl: f32,
    /// Heading size (24px)
    pub heading: f32,
    /// Title/display size (32px)
    pub title: f32,
}

impl TypeScale {
    /// Create a type scale from a base font size
    pub fn from_base(base: f32) -> Self {
        Self {
            xs: base * 0.714,     // ~10px at base 14
            sm: base * 0.857,     // ~12px at base 14
            base,                 // 14px
            md: base * 1.143,     // ~16px at base 14
            lg: base * 1.286,     // ~18px at base 14
            xl: base * 1.429,     // ~20px at base 14
            xxl: base * 1.714,    // ~24px at base 14
            heading: base * 1.714, // ~24px at base 14
            title: base * 2.286,  // ~32px at base 14
        }
    }

    /// Scale all sizes by a factor
    pub fn scaled(self, factor: f32) -> Self {
        Self {
            xs: self.xs * factor,
            sm: self.sm * factor,
            base: self.base * factor,
            md: self.md * factor,
            lg: self.lg * factor,
            xl: self.xl * factor,
            xxl: self.xxl * factor,
            heading: self.heading * factor,
            title: self.title * factor,
        }
    }
}

impl Default for TypeScale {
    fn default() -> Self {
        Self::from_base(14.0)
    }
}

/// Line height multipliers for different text types
#[derive(Clone, Debug)]
pub struct LineHeights {
    /// Tight line height for compact text
    pub tight: f32,
    /// Normal line height for body text
    pub normal: f32,
    /// Relaxed line height for readable text
    pub relaxed: f32,
    /// Loose line height for large text
    pub loose: f32,
}

impl Default for LineHeights {
    fn default() -> Self {
        Self {
            tight: 1.25,
            normal: 1.5,
            relaxed: 1.75,
            loose: 2.0,
        }
    }
}

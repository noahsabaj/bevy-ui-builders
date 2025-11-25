//! Border settings for UI elements.

/// Border settings for the theme.
#[derive(Clone, Debug)]
pub struct ThemeBorders {
    /// Border width values
    pub width: BorderWidths,
    /// Border radius values
    pub radius: BorderRadii,
}

impl Default for ThemeBorders {
    fn default() -> Self {
        Self {
            width: BorderWidths::default(),
            radius: BorderRadii::default(),
        }
    }
}

/// Border width values
#[derive(Clone, Debug)]
pub struct BorderWidths {
    /// No border (0px)
    pub none: f32,
    /// Thin border (1px)
    pub thin: f32,
    /// Default border (2px)
    pub default: f32,
    /// Medium border (2px)
    pub medium: f32,
    /// Thick border (3px)
    pub thick: f32,
}

impl Default for BorderWidths {
    fn default() -> Self {
        Self {
            none: 0.0,
            thin: 1.0,
            default: 2.0,
            medium: 2.0,
            thick: 3.0,
        }
    }
}

/// Border radius values
#[derive(Clone, Debug)]
pub struct BorderRadii {
    /// No radius (sharp corners)
    pub none: f32,
    /// Small radius (4px)
    pub sm: f32,
    /// Medium radius (6px)
    pub md: f32,
    /// Large radius (8px)
    pub lg: f32,
    /// Extra large radius (12px)
    pub xl: f32,
    /// Full/pill radius (9999px)
    pub full: f32,
}

impl Default for BorderRadii {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 6.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

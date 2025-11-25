//! Label component types and markers

use bevy::prelude::*;

use crate::theme::{SemanticVariant, UiTheme};

/// Component for text labels
#[derive(Component, Debug)]
pub struct Label {
    /// Size/typography style of the label
    pub size: LabelSize,
    /// Optional semantic variant for colored labels
    pub variant: Option<SemanticVariant>,
}

/// Label size/typography variants (controls font size only)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelSize {
    /// Large title text (32px)
    Title,
    /// Section heading (24px)
    Heading,
    /// Normal body text (default, 16px)
    #[default]
    Body,
    /// Small caption text (12px)
    Caption,
    /// Small muted/secondary text (12px)
    Small,
}

impl LabelSize {
    /// Get the font size for this label size from theme
    pub fn font_size(&self, theme: &UiTheme) -> f32 {
        match self {
            LabelSize::Title => theme.typography.scale.title,
            LabelSize::Heading => theme.typography.scale.heading,
            LabelSize::Body => theme.typography.scale.md,
            LabelSize::Caption => theme.typography.scale.sm,
            LabelSize::Small => theme.typography.scale.sm,
        }
    }

    /// Get the default text color for this label size from theme
    /// (used when no semantic variant is specified)
    pub fn default_text_color(&self, theme: &UiTheme) -> Color {
        match self {
            LabelSize::Title => theme.colors.text.title,
            LabelSize::Heading => theme.colors.text.primary,
            LabelSize::Body => theme.colors.text.secondary,
            LabelSize::Caption => theme.colors.text.muted,
            LabelSize::Small => theme.colors.text.muted,
        }
    }
}

// Keep LabelStyle as alias for backwards compatibility during migration
/// Legacy alias for LabelSize (deprecated, use LabelSize instead)
#[deprecated(since = "1.0.0", note = "Use LabelSize for sizing, SemanticVariant for colors")]
pub type LabelStyle = LabelSize;
//! Label component types and markers

use bevy::prelude::*;

/// Component for text labels
#[derive(Component, Debug)]
pub struct Label {
    pub style: LabelStyle,
}

/// Label style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelStyle {
    Title,   // Large title text
    Heading, // Section heading
    #[default]
    Body, // Normal body text
    Caption, // Small caption text
    Muted,   // De-emphasized text
    Error,   // Error message text
    Success, // Success message text
    Warning, // Warning message text
}

impl LabelStyle {
    pub fn font_size(&self) -> f32 {
        match self {
            LabelStyle::Title => crate::dimensions::FONT_SIZE_TITLE,
            LabelStyle::Heading => crate::dimensions::FONT_SIZE_LARGE,
            LabelStyle::Body => crate::dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Caption => crate::dimensions::FONT_SIZE_SMALL,
            LabelStyle::Muted => crate::dimensions::FONT_SIZE_SMALL,
            LabelStyle::Error => crate::dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Success => crate::dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Warning => crate::dimensions::FONT_SIZE_NORMAL,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            LabelStyle::Title => crate::colors::TEXT_TITLE,
            LabelStyle::Heading => crate::colors::TEXT_PRIMARY,
            LabelStyle::Body => crate::colors::TEXT_SECONDARY,
            LabelStyle::Caption => crate::colors::TEXT_MUTED,
            LabelStyle::Muted => crate::colors::TEXT_MUTED,
            LabelStyle::Error => crate::colors::DANGER,
            LabelStyle::Success => crate::colors::SUCCESS,
            LabelStyle::Warning => crate::colors::WARNING,
        }
    }
}
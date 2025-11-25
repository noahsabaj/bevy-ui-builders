//! LabelBuilder implementation

use bevy::prelude::*;

use super::types::{Label as UiLabel, LabelSize};
use crate::theme::{SemanticVariant, UiTheme};
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

// Default colors (dark theme) for when no theme is provided
mod defaults {
    use bevy::prelude::Color;

    pub const TEXT_TITLE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);

    pub const FONT_TITLE: f32 = 32.0;
    pub const FONT_HEADING: f32 = 24.0;
    pub const FONT_BODY: f32 = 16.0;
    pub const FONT_SMALL: f32 = 12.0;

    pub const SUCCESS: Color = Color::srgb(0.25, 0.76, 0.25);
    pub const WARNING: Color = Color::srgb(0.96, 0.76, 0.05);
    pub const DANGER: Color = Color::srgb(0.86, 0.25, 0.25);
}

/// Resolved colors and sizes for a label (either from theme or defaults)
struct ResolvedLabelStyle {
    font_size: f32,
    text_color: Color,
}

/// Builder for creating labels with consistent styling
pub struct LabelBuilder {
    text: String,
    size: LabelSize,
    variant: Option<SemanticVariant>,
    font_size: Option<f32>,
    text_color: Option<Color>,
    text_align: JustifyContent,
    // Theme-resolved values (set via .themed())
    themed_font_size: Option<f32>,
    themed_text_color: Option<Color>,
    base: BuilderBase,
}

impl LabelBuilder {
    /// Create a new label builder
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: LabelSize::Body,
            variant: None,
            font_size: None,
            text_color: None,
            text_align: JustifyContent::Start,
            themed_font_size: None,
            themed_text_color: None,
            base: BuilderBase::new(),
        }
    }

    /// Apply theme colors and sizes to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    ///
    /// # Example
    /// ```ignore
    /// fn setup(mut commands: Commands, theme: Res<UiTheme>) {
    ///     commands.spawn(Node::default()).with_children(|parent| {
    ///         LabelBuilder::new("Themed!")
    ///             .themed(&theme)
    ///             .variant(SemanticVariant::Success)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        // Pre-resolve theme values
        self.themed_font_size = Some(self.size.font_size(theme));
        self.themed_text_color = Some(self.resolve_text_color_from_theme(theme));
        self
    }

    /// Resolve text color from theme based on size and variant
    fn resolve_text_color_from_theme(&self, theme: &UiTheme) -> Color {
        if let Some(variant) = self.variant {
            match variant {
                SemanticVariant::Primary => theme.colors.primary.base,
                SemanticVariant::Secondary => theme.colors.text.secondary,
                SemanticVariant::Success => theme.colors.success.base,
                SemanticVariant::Warning => theme.colors.warning.base,
                SemanticVariant::Danger => theme.colors.danger.base,
                SemanticVariant::Ghost => theme.colors.text.secondary,
            }
        } else {
            self.size.default_text_color(theme)
        }
    }

    /// Set the label size (controls font size)
    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    /// Set the label style (alias for size, for backwards compatibility)
    #[deprecated(since = "1.0.0", note = "Use size() for sizing, variant() for colors")]
    pub fn style(mut self, style: LabelSize) -> Self {
        self.size = style;
        self
    }

    /// Set the semantic variant (controls color)
    ///
    /// # Example
    /// ```ignore
    /// LabelBuilder::new("Error!")
    ///     .size(LabelSize::Body)
    ///     .variant(SemanticVariant::Danger)
    ///     .build(parent);
    /// ```
    pub fn variant(mut self, variant: SemanticVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Override the font size directly
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Override the text color directly
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set text alignment
    pub fn text_align(mut self, align: JustifyContent) -> Self {
        self.text_align = align;
        self
    }

    /// Resolve final styles (priority: override > themed > default)
    fn resolve_styles(&self) -> ResolvedLabelStyle {
        // Font size priority: direct override > themed > default
        let font_size = self.font_size
            .or(self.themed_font_size)
            .unwrap_or_else(|| self.default_font_size());

        // Text color priority: direct override > themed > variant default > size default
        let text_color = self.text_color
            .or(self.themed_text_color)
            .unwrap_or_else(|| self.default_text_color());

        ResolvedLabelStyle { font_size, text_color }
    }

    /// Get default font size (no theme)
    fn default_font_size(&self) -> f32 {
        match self.size {
            LabelSize::Title => defaults::FONT_TITLE,
            LabelSize::Heading => defaults::FONT_HEADING,
            LabelSize::Body => defaults::FONT_BODY,
            LabelSize::Caption => defaults::FONT_SMALL,
            LabelSize::Small => defaults::FONT_SMALL,
        }
    }

    /// Get default text color (no theme)
    fn default_text_color(&self) -> Color {
        // If variant is set, use variant-specific color
        if let Some(variant) = self.variant {
            match variant {
                SemanticVariant::Success => defaults::SUCCESS,
                SemanticVariant::Warning => defaults::WARNING,
                SemanticVariant::Danger => defaults::DANGER,
                _ => self.default_size_color(),
            }
        } else {
            self.default_size_color()
        }
    }

    /// Get default color based on size (no variant, no theme)
    fn default_size_color(&self) -> Color {
        match self.size {
            LabelSize::Title => defaults::TEXT_TITLE,
            LabelSize::Heading => defaults::TEXT_PRIMARY,
            LabelSize::Body => defaults::TEXT_SECONDARY,
            LabelSize::Caption => defaults::TEXT_MUTED,
            LabelSize::Small => defaults::TEXT_MUTED,
        }
    }
}

impl UiBuilder for LabelBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        let styles = self.resolve_styles();

        let entity = parent
            .spawn((
                Text::new(self.text.clone()),
                TextFont {
                    font_size: styles.font_size,
                    ..default()
                },
                TextColor(styles.text_color),
                Node {
                    margin: self.base.node.margin,
                    ..default()
                },
                UiLabel {
                    size: self.size,
                    variant: self.variant,
                },
            ))
            .id();

        self.base.apply(entity, &mut parent.commands());
        entity
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for LabelBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(self, _padding: UiRect) -> Self {
        self
    }

    fn width(self, _width: Val) -> Self {
        self
    }

    fn height(self, _height: Val) -> Self {
        self
    }
}

/// Convenience function to create a label builder
pub fn label(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text)
}

/// Convenience function to create a heading label
pub fn heading(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).size(LabelSize::Heading)
}

/// Convenience function to create a title label
pub fn title(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).size(LabelSize::Title)
}

/// Convenience function to create secondary/muted text
pub fn secondary_text(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).size(LabelSize::Small)
}

/// Convenience function to create an error label
pub fn error_label(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).variant(SemanticVariant::Danger)
}

/// Convenience function to create a success label
pub fn success_label(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).variant(SemanticVariant::Success)
}

/// Convenience function to create a warning label
pub fn warning_label(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text).variant(SemanticVariant::Warning)
}
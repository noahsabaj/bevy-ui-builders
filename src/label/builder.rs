//! LabelBuilder implementation

use bevy::prelude::*;
use super::types::*;

/// Builder for creating labels with consistent styling
pub struct LabelBuilder {
    text: String,
    style: LabelStyle,
    font_size: Option<f32>,
    color: Option<Color>,
    margin: UiRect,
    text_align: JustifyContent,
}

impl LabelBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: LabelStyle::Body,
            font_size: None,
            color: None,
            margin: UiRect::all(Val::Px(0.0)),
            text_align: JustifyContent::Start,
        }
    }

    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }

    /// Override the font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Override the text color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    /// Set text alignment
    pub fn text_align(mut self, align: JustifyContent) -> Self {
        self.text_align = align;
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let font_size = self.font_size.unwrap_or_else(|| self.style.font_size());
        let text_color = self.color.unwrap_or_else(|| self.style.text_color());

        parent
            .spawn((
                Node {
                    margin: self.margin,
                    justify_content: self.text_align,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|container| {
                // The actual text entity
                container.spawn((
                    Text::new(self.text.clone()),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(text_color),
                    super::types::Label { style: self.style },
                ));
            })
            .id()
    }
}

/// Convenience function to create a label builder
pub fn label(text: impl Into<String>) -> LabelBuilder {
    LabelBuilder::new(text)
}
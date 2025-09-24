//! ProgressBarBuilder implementation

use bevy::prelude::*;
use crate::{colors, dimensions};
use super::types::*;

/// Builder for creating progress bars with consistent styling
pub struct ProgressBarBuilder {
    value: f32,
    style: ProgressBarStyle,
    width: Val,
    height: Option<f32>,
    margin: UiRect,
    track_color: Option<Color>,
    fill_color: Option<Color>,
    show_label: bool,
    custom_label: Option<String>,
    animated: bool,
}

impl ProgressBarBuilder {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            style: ProgressBarStyle::Default,
            width: Val::Percent(100.0),
            height: None,
            margin: UiRect::all(Val::Px(0.0)),
            track_color: None,
            fill_color: None,
            show_label: false,
            custom_label: None,
            animated: false,
        }
    }

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Override the height
    pub fn height(mut self, height: Val) -> Self {
        if let Val::Px(px) = height {
            self.height = Some(px);
        }
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    /// Override the track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Override the fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Show a percentage label
    pub fn with_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    /// Show a custom label text instead of percentage
    pub fn with_label_text(mut self, text: impl Into<String>) -> Self {
        self.custom_label = Some(text.into());
        self.show_label = true;
        self
    }

    /// Enable animation
    pub fn animated(mut self) -> Self {
        self.animated = true;
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let height = Val::Px(self.height.unwrap_or_else(|| self.style.height()));
        let track_color = self.track_color.unwrap_or_else(|| self.style.track_color());
        let fill_color = self.fill_color.unwrap_or_else(|| self.style.fill_color());

        parent
            .spawn((
                Node {
                    width: self.width,
                    flex_direction: FlexDirection::Column,
                    margin: self.margin,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|container| {
                // Progress bar track (background)
                container
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height,
                            position_type: PositionType::Relative,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(track_color),
                        BorderRadius::all(Val::Px(2.0)),
                        ProgressBarTrack,
                    ))
                    .with_children(|track| {
                        // Progress bar fill
                        track.spawn((
                            Node {
                                width: Val::Percent(self.value * 100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            BackgroundColor(fill_color),
                            BorderRadius::all(Val::Px(2.0)),
                            ProgressBarFill,
                        ));
                    });

                // Optional label
                if self.show_label {
                    let label_text = self
                        .custom_label
                        .unwrap_or_else(|| format!("{}%", (self.value * 100.0) as i32));

                    container
                        .spawn((
                            Node {
                                margin: UiRect::top(Val::Px(4.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ))
                        .with_children(|label_container| {
                            label_container.spawn((
                                Text::new(label_text),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_SMALL,
                                    ..default()
                                },
                                TextColor(colors::TEXT_MUTED),
                                ProgressBarLabel,
                            ));
                        });
                }
            })
            .insert(ProgressBar {
                value: self.value,
                style: self.style,
                animated: self.animated,
            })
            .id()
    }
}

/// Convenience function to create a progress bar builder
pub fn progress(value: f32) -> ProgressBarBuilder {
    ProgressBarBuilder::new(value)
}
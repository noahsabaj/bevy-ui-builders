//! SliderBuilder implementation

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::button::{ButtonBuilder, ButtonSize, ButtonStyle};
use crate::styles::{colors, dimensions};
use crate::systems::hover::{HoverScale, HoverColors};
use super::types::*;

/// Builder for creating sliders
pub struct SliderBuilder {
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    width: Val,
    format: ValueFormat,
    with_preview: bool,
    with_buttons: bool,
    label: Option<String>,
}

impl SliderBuilder {
    /// Create a new slider builder with range
    pub fn new(range: std::ops::Range<f32>) -> Self {
        Self {
            value: range.start,
            min: range.start,
            max: range.end,
            step: None,
            width: Val::Px(200.0),
            format: ValueFormat::Decimal(1),
            with_preview: true,
            with_buttons: false,
            label: None,
        }
    }

    /// Set the initial value
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Set the step size for snapping
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set the width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Set the value format
    pub fn format(mut self, format: ValueFormat) -> Self {
        self.format = format;
        self
    }

    /// Show/hide value preview
    pub fn with_preview(mut self, show: bool) -> Self {
        self.with_preview = show;
        self
    }

    /// Add increment/decrement buttons
    pub fn with_buttons(mut self) -> Self {
        self.with_buttons = true;
        self
    }

    /// Add a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Build the slider
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let container = parent.spawn((
            Node {
                width: self.width,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                margin: UiRect::bottom(Val::Px(dimensions::SPACING_MEDIUM)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).id();

        let mut value_text_id = None;

        parent.commands().entity(container).with_children(|container| {
            // Label and value row
            if self.label.is_some() || self.with_preview {
                container.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|row| {
                    // Label
                    if let Some(label) = self.label {
                        row.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_MEDIUM,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            SliderLabel,
                        ));
                    }

                    // Value text
                    if self.with_preview {
                        let entity = row.spawn((
                            Text::new(self.format.format(self.value)),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_MEDIUM,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            SliderValueText,
                        )).id();
                        value_text_id = Some(entity);
                    }
                });
            }

            // Slider track and handle
            let mut slider_entity = container.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(dimensions::SLIDER_TRACK_HEIGHT + dimensions::SLIDER_HANDLE_SIZE),
                    padding: UiRect::vertical(Val::Px(
                        (dimensions::SLIDER_HANDLE_SIZE - dimensions::SLIDER_TRACK_HEIGHT) / 2.0
                    )),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Interaction::default(),
                RelativeCursorPosition::default(),
                SliderTrack,
                // Note: Cursor hover effects not available in Bevy 0.16
            ));

            let track_entity = slider_entity.id();

            let mut slider = Slider::new(self.min, self.max, self.value);
            slider.step = self.step;
            slider.value_text_entity = value_text_id;

            slider_entity.insert(slider.clone());
            slider_entity.insert(SliderConfig {
                show_value: self.with_preview,
                value_format: self.format.clone(),
                track_height: dimensions::SLIDER_TRACK_HEIGHT,
                handle_size: dimensions::SLIDER_HANDLE_SIZE,
                track_color: colors::BACKGROUND_TERTIARY,
                fill_color: colors::PRIMARY.with_alpha(0.3),
                handle_color: colors::PRIMARY,
            });

            slider_entity.with_children(|track| {
                // Track background
                track.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(dimensions::SLIDER_TRACK_HEIGHT),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_TERTIARY),
                    BorderRadius::all(Val::Px(dimensions::SLIDER_TRACK_HEIGHT / 2.0)),
                ));

                // Filled portion
                let fill_width = slider.normalized() * 100.0;
                track.spawn((
                    Node {
                        width: Val::Percent(fill_width),
                        height: Val::Px(dimensions::SLIDER_TRACK_HEIGHT),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY.with_alpha(0.3)),
                    BorderRadius::all(Val::Px(dimensions::SLIDER_TRACK_HEIGHT / 2.0)),
                    SliderFill,
                ));

                // Handle
                let handle_offset = slider.normalized() * 100.0;
                track.spawn((
                    Node {
                        width: Val::Px(dimensions::SLIDER_HANDLE_SIZE),
                        height: Val::Px(dimensions::SLIDER_HANDLE_SIZE),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(handle_offset.min(95.0)),
                        top: Val::Px(0.0),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_MEDIUM)),
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    BorderColor(colors::BORDER_LIGHT),
                    BorderRadius::all(Val::Px(dimensions::SLIDER_HANDLE_SIZE / 2.0)),
                    SliderHandle,
                    // Add hover effects for the handle
                    HoverScale(1.15), // Grow 15% on hover
                    HoverColors {
                        normal_bg: colors::PRIMARY,
                        hover_bg: colors::PRIMARY_HOVER,
                        normal_border: colors::BORDER_LIGHT,
                        hover_border: colors::BORDER_FOCUS,
                    },
                    Interaction::default(),
                ));
            });

            // Add increment/decrement buttons if requested
            if self.with_buttons {
                container.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(dimensions::SPACING_MEDIUM),
                        margin: UiRect::top(Val::Px(dimensions::SPACING_SMALL)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|button_row| {
                    // Decrement button
                    let dec_button = ButtonBuilder::new("-")
                        .style(ButtonStyle::Secondary)
                        .size(ButtonSize::Small)
                        .build(button_row);

                    button_row.commands()
                        .entity(dec_button)
                        .insert(SliderButtonAction {
                            slider_entity: track_entity,
                            delta: -self.step.unwrap_or((self.max - self.min) / 100.0),
                        });

                    // Increment button
                    let inc_button = ButtonBuilder::new("+")
                        .style(ButtonStyle::Secondary)
                        .size(ButtonSize::Small)
                        .build(button_row);

                    button_row.commands()
                        .entity(inc_button)
                        .insert(SliderButtonAction {
                            slider_entity: track_entity,
                            delta: self.step.unwrap_or((self.max - self.min) / 100.0),
                        });
                });
            }
        });

        container
    }
}
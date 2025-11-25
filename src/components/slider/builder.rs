//! SliderBuilder implementation

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::animation::AnimationCategory;
use crate::components::button::{ButtonBuilder, ButtonSize, ButtonStyle};
use crate::styles::dimensions;
use crate::theme::{UiTheme, SemanticVariant};
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating sliders
pub struct SliderBuilder {
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    format: ValueFormat,
    variant: SemanticVariant,
    with_preview: bool,
    with_buttons: bool,
    label: Option<String>,
    // Theme-resolved colors (set via .themed())
    themed_colors: Option<SliderColors>,
    base: BuilderBase,
}

impl SliderBuilder {
    /// Create a new slider builder with range
    pub fn new(range: std::ops::Range<f32>) -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Px(200.0);
        base.node.flex_direction = FlexDirection::Column;
        base.node.row_gap = Val::Px(dimensions::SPACING_SMALL);
        base.node.margin = UiRect::bottom(Val::Px(dimensions::SPACING_MEDIUM));

        Self {
            value: range.start,
            min: range.start,
            max: range.end,
            step: None,
            format: ValueFormat::Decimal(1),
            variant: SemanticVariant::Primary,
            with_preview: true,
            with_buttons: false,
            label: None,
            themed_colors: None,
            base,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    ///
    /// # Example
    /// ```ignore
    /// fn setup(mut commands: Commands, theme: Res<UiTheme>) {
    ///     commands.spawn(Node::default()).with_children(|parent| {
    ///         SliderBuilder::new(0.0..100.0)
    ///             .themed(&theme)
    ///             .variant(SemanticVariant::Success)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(SliderColors::from_theme(theme, self.variant));
        self
    }

    /// Set the semantic variant (controls colors)
    pub fn variant(mut self, variant: SemanticVariant) -> Self {
        self.variant = variant;
        self
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
        self.base.node.width = width;
        self
    }

    /// Set the value format
    pub fn format(mut self, format: ValueFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the value format (alias for format)
    pub fn with_format(self, format: ValueFormat) -> Self {
        self.format(format)
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

    /// Build the slider (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> SliderColors {
        self.themed_colors.clone()
            .unwrap_or_else(|| SliderColors::default_colors(self.variant))
    }
}

impl UiBuilder for SliderBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Resolve colors (themed > default)
        let colors = self.resolve_colors();

        let container = parent.spawn((
            self.base.node,
            BackgroundColor(Color::NONE),
        )).id();

        let mut value_text_id = None;
        let label = self.label.clone();
        let with_preview = self.with_preview;
        let format = self.format.clone();
        let value = self.value;
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let with_buttons = self.with_buttons;

        parent.commands().entity(container).with_children(|container| {
            // Label and value row
            if label.is_some() || with_preview {
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
                    if let Some(label_text) = label.clone() {
                        row.spawn((
                            Text::new(label_text),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_MEDIUM,
                                ..default()
                            },
                            TextColor(colors.text_label),
                            SliderLabel,
                        ));
                    }

                    // Value text
                    if with_preview {
                        let entity = row.spawn((
                            Text::new(format.format(value)),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_MEDIUM,
                                ..default()
                            },
                            TextColor(colors.text_value),
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
                    padding: UiRect {
                        left: Val::Px(dimensions::SLIDER_HANDLE_SIZE / 2.0),  // Half handle width
                        right: Val::Px(dimensions::SLIDER_HANDLE_SIZE / 2.0), // Half handle width
                        top: Val::Px((dimensions::SLIDER_HANDLE_SIZE - dimensions::SLIDER_TRACK_HEIGHT) / 2.0),
                        bottom: Val::Px((dimensions::SLIDER_HANDLE_SIZE - dimensions::SLIDER_TRACK_HEIGHT) / 2.0),
                    },
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

            let mut slider = Slider::new(min, max, value);
            slider.step = step;
            slider.value_text_entity = value_text_id;

            slider_entity.insert(slider.clone());
            slider_entity.insert(SliderConfig {
                show_value: with_preview,
                value_format: format.clone(),
                track_height: dimensions::SLIDER_TRACK_HEIGHT,
                handle_size: dimensions::SLIDER_HANDLE_SIZE,
                track_color: colors.track,
                fill_color: colors.fill,
                handle_color: colors.handle,
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
                    BackgroundColor(colors.track),
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
                    BackgroundColor(colors.fill),
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
                        left: Val::Percent(handle_offset),
                        top: Val::Px(0.0),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_MEDIUM)),
                        ..default()
                    },
                    BackgroundColor(colors.handle),
                    BorderColor::all(colors.handle_border),
                    BorderRadius::all(Val::Px(dimensions::SLIDER_HANDLE_SIZE / 2.0)),
                    SliderHandle,
                    Transform::default(), // Required for scale animations
                    AnimationCategory::Slider, // Auto-animation with slider defaults (1.15 scale)
                    Interaction::default(),
                ));
            });

            // Add increment/decrement buttons if requested
            if with_buttons {
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
                            delta: -step.unwrap_or((max - min) / 100.0),
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
                            delta: step.unwrap_or((max - min) / 100.0),
                        });
                });
            }
        });

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(container));
        }

        container
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

impl LayoutBuilder for SliderBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }
}

/// Convenience function to create a slider with a range
pub fn slider(min: f32, max: f32) -> SliderBuilder {
    SliderBuilder::new(min..max)
}

/// Convenience function to create a slider from 0 to 100 (percentage)
pub fn percentage_slider() -> SliderBuilder {
    SliderBuilder::new(0.0..100.0)
        .step(1.0)
        .format(ValueFormat::Percentage)
}

/// Convenience function to create a slider from 0 to 1 (normalized)
pub fn normalized_slider() -> SliderBuilder {
    SliderBuilder::new(0.0..1.0)
        .step(0.01)
}
//! ButtonBuilder implementation

use bevy::prelude::*;
use crate::styles::{colors, dimensions, ButtonStyle, ButtonSize};
use super::types::{StyledButton, ButtonStateColors};
use crate::systems::hover::{HoverScale, HoverBrightness, OriginalColors};

/// Builder for creating buttons with consistent styling
pub struct ButtonBuilder {
    text: String,
    style: ButtonStyle,
    size: ButtonSize,
    width: Option<Val>,
    custom_height: Option<Val>,
    margin: Option<UiRect>,
    hover_scale: Option<f32>,
    hover_brightness: Option<f32>,
    disabled: bool,
    icon: Option<String>,
}

impl ButtonBuilder {
    /// Create a new button builder with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::Primary,
            size: ButtonSize::Medium,
            width: None,
            custom_height: None,
            margin: None,
            hover_scale: None,
            hover_brightness: None,
            disabled: false,
            icon: None,
        }
    }

    /// Set the button style
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the button size
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set a custom width
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Set a custom height
    pub fn height(mut self, height: Val) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Set the margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Enable hover scale effect
    pub fn hover_scale(mut self, scale: f32) -> Self {
        self.hover_scale = Some(scale);
        self
    }

    /// Enable hover brightness effect
    pub fn hover_brightness(mut self, brightness: f32) -> Self {
        self.hover_brightness = Some(brightness);
        self
    }

    /// Set the button as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Set whether the button is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.disabled = !enabled;
        self
    }

    /// Add an icon (emoji or symbol)
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Attach a marker component to the button
    pub fn with_marker<M: Component>(self, marker: M) -> ButtonBuilderWithMarker<M> {
        ButtonBuilderWithMarker {
            builder: self,
            marker,
        }
    }

    /// Build the button entity (alias for build)
    pub fn build_in(self, parent: &mut ChildSpawnerCommands) -> Entity {
        self.build(parent)
    }

    /// Build the button entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let (bg_color, border_color, text_color) = get_style_colors(&self.style, self.disabled);
        let (width, height) = get_size_dimensions(&self.size);
        let font_size = get_font_size(&self.size);

        let button_width = self.width.unwrap_or(Val::Px(width));
        let button_height = self.custom_height.unwrap_or(Val::Px(height));
        let button_margin = self.margin.unwrap_or_default();

        let mut button = parent.spawn((
            Button,
            Node {
                width: button_width,
                height: button_height,
                margin: button_margin,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_MEDIUM)),
                padding: UiRect::horizontal(Val::Px(dimensions::PADDING_MEDIUM)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_MEDIUM)),
            StyledButton,
        ));

        // Store state colors for automatic hover effects
        button.insert(ButtonStateColors {
            normal_bg: bg_color,
            hover_bg: self.style.hover_color(),
            pressed_bg: self.style.pressed_color(),
            normal_border: border_color,
            hover_border: self.style.border_color(),
            pressed_border: self.style.border_color(),
        });

        // Store original colors for custom hover effects
        button.insert(OriginalColors {
            background: bg_color,
            border: border_color,
        });

        // Add hover scale - use default if not specified
        let scale = self.hover_scale.unwrap_or(1.015); // Even more subtle default scale
        button.insert(HoverScale(scale));

        // Add animation state for smooth transitions
        button.insert(super::types::ButtonAnimationState {
            current_scale: 1.0,
            target_scale: 1.0,
            current_color_blend: 0.0,
            target_color_blend: 0.0,
            animation_speed: 12.0, // Smooth but responsive
        });

        // Add hover brightness if specified (optional)
        if let Some(brightness) = self.hover_brightness {
            button.insert(HoverBrightness(brightness));
        }

        if self.disabled {
            button.insert(Interaction::None);
        }

        let button_entity = button.id();

        // Add text content
        button.with_children(|button| {
            if let Some(icon) = self.icon {
                // Icon + Text layout
                button.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(dimensions::SPACING_SMALL),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|container| {
                    // Icon
                    container.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(text_color),
                    ));

                    // Text
                    container.spawn((
                        Text::new(&self.text),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            } else {
                // Just text
                button.spawn((
                    Text::new(&self.text),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(text_color),
                ));
            }
        });

        button_entity
    }
}

/// Get colors for a button style
fn get_style_colors(style: &ButtonStyle, disabled: bool) -> (Color, Color, Color) {
    if disabled {
        return (
            colors::BACKGROUND_TERTIARY,
            colors::BORDER_DEFAULT,
            colors::TEXT_DISABLED,
        );
    }

    match style {
        ButtonStyle::Primary => (colors::PRIMARY, colors::PRIMARY_DARK, colors::TEXT_ON_PRIMARY),
        ButtonStyle::Secondary => (colors::SECONDARY, colors::SECONDARY_DARK, colors::TEXT_ON_SECONDARY),
        ButtonStyle::Success => (colors::SUCCESS, colors::SUCCESS_DARK, colors::TEXT_ON_SUCCESS),
        ButtonStyle::Danger => (colors::DANGER, colors::DANGER_DARK, colors::TEXT_ON_DANGER),
        ButtonStyle::Warning => (colors::WARNING, colors::WARNING_PRESSED, Color::BLACK),
        ButtonStyle::Ghost => (Color::NONE, colors::BORDER_DEFAULT, colors::TEXT_PRIMARY),
    }
}

/// Get dimensions for a button size
fn get_size_dimensions(size: &ButtonSize) -> (f32, f32) {
    match size {
        ButtonSize::Small => (dimensions::BUTTON_WIDTH_SMALL, dimensions::BUTTON_HEIGHT_SMALL),
        ButtonSize::Medium => (dimensions::BUTTON_WIDTH_MEDIUM, dimensions::BUTTON_HEIGHT_MEDIUM),
        ButtonSize::Large => (dimensions::BUTTON_WIDTH_LARGE, dimensions::BUTTON_HEIGHT_LARGE),
        ButtonSize::XLarge => (dimensions::BUTTON_WIDTH_XLARGE, dimensions::BUTTON_HEIGHT_XLARGE),
    }
}

/// Get font size for a button size
fn get_font_size(size: &ButtonSize) -> f32 {
    match size {
        ButtonSize::Small => dimensions::FONT_SIZE_SMALL,
        ButtonSize::Medium => dimensions::FONT_SIZE_MEDIUM,
        ButtonSize::Large => dimensions::FONT_SIZE_LARGE,
        ButtonSize::XLarge => dimensions::FONT_SIZE_XLARGE,
    }
}

/// Convenience function for creating a primary button
pub fn primary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Primary)
}

/// Convenience function for creating a secondary button
pub fn secondary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Secondary)
}

/// Convenience function for creating a success button
pub fn success_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Success)
}

/// Convenience function for creating a danger button
pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Danger)
}

/// Convenience function for creating a ghost button
pub fn ghost_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Ghost)
}

/// A ButtonBuilder with an attached marker component
pub struct ButtonBuilderWithMarker<M: Component> {
    builder: ButtonBuilder,
    marker: M,
}

impl<M: Component> ButtonBuilderWithMarker<M> {
    /// Build the button with the marker component
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let entity = self.builder.build(parent);
        parent.commands().entity(entity).insert(self.marker);
        entity
    }

    /// Build the button with the marker component (alias for build)
    pub fn build_in(self, parent: &mut ChildSpawnerCommands) -> Entity {
        self.build(parent)
    }
}
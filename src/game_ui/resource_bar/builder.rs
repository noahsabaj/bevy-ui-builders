//! ResourceBarBuilder implementation

use bevy::prelude::*;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating resource bars (health, mana, stamina, etc.)
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::game_ui::resource_bar::*;
///
/// // Basic health bar
/// ResourceBarBuilder::new()
///     .value(80.0)
///     .max_value(100.0)
///     .style(ResourceBarStyle::Health)
///     .build(parent);
///
/// // Custom styled bar
/// ResourceBarBuilder::new()
///     .value(50.0)
///     .max_value(100.0)
///     .fill_color(Color::srgb(0.2, 0.8, 0.2))
///     .height(16.0)
///     .animated(true)
///     .build(parent);
/// ```
pub struct ResourceBarBuilder {
    value: f32,
    max_value: f32,
    style: ResourceBarStyle,
    config: ResourceBarConfig,
    animated: bool,
    show_text: bool,
    width: Val,
    base: BuilderBase,
}

impl ResourceBarBuilder {
    /// Create a new resource bar builder
    pub fn new() -> Self {
        Self {
            value: 100.0,
            max_value: 100.0,
            style: ResourceBarStyle::Health,
            config: ResourceBarConfig::default(),
            animated: true,
            show_text: false,
            width: Val::Px(200.0),
            base: BuilderBase::new(),
        }
    }

    /// Set the current value
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the maximum value
    pub fn max_value(mut self, max: f32) -> Self {
        self.max_value = max;
        self
    }

    /// Set the visual style preset
    pub fn style(mut self, style: ResourceBarStyle) -> Self {
        self.style = style;
        self.config.fill_color = style.fill_color();
        self.config.background_color = style.background_color();
        self.config.damage_color = style.damage_color();
        self
    }

    /// Set custom fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.config.fill_color = color;
        self.style = ResourceBarStyle::Custom;
        self
    }

    /// Set custom background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.config.background_color = color;
        self
    }

    /// Set the bar height
    pub fn height(mut self, height: f32) -> Self {
        self.config.height = height;
        self
    }

    /// Set the bar width
    pub fn bar_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Enable or disable animation
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Show or hide the damage indicator
    pub fn show_damage_indicator(mut self, show: bool) -> Self {
        self.config.show_damage_indicator = show;
        self
    }

    /// Show the current/max value as text
    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.config.corner_radius = radius;
        self
    }

    /// Set animation speed
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.config.animation_speed = speed;
        self
    }
}

impl Default for ResourceBarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for ResourceBarBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let percentage = if self.max_value > 0.0 {
            (self.value / self.max_value).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Pre-spawn bar entity
        let bar_entity = parent.spawn_empty().id();

        parent.commands().entity(bar_entity).insert((
            Node {
                width: self.width,
                height: Val::Px(self.config.height),
                border: UiRect::all(Val::Px(self.config.border_width)),
                ..default()
            },
            BackgroundColor(self.config.background_color),
            BorderColor::all(self.config.border_color),
            BorderRadius::all(Val::Px(self.config.corner_radius)),
            ResourceBar {
                value: self.value,
                max_value: self.max_value,
                style: self.style,
                animated: self.animated,
            },
        )).with_children(|bar| {
            // Damage indicator (behind the fill)
            if self.config.show_damage_indicator {
                bar.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(percentage * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(self.config.damage_color),
                    BorderRadius::all(Val::Px(self.config.corner_radius - self.config.border_width)),
                    ResourceBarDamageIndicator {
                        bar: bar_entity,
                        display_percentage: percentage,
                        delay_timer: 0.0,
                    },
                ));
            }

            // Main fill
            bar.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(percentage * 100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(self.config.fill_color),
                BorderRadius::all(Val::Px(self.config.corner_radius - self.config.border_width)),
                ResourceBarFill {
                    bar: bar_entity,
                    target_percentage: percentage,
                    display_percentage: percentage,
                },
            ));

            // Optional text overlay
            if self.show_text {
                bar.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(format!("{:.0}/{:.0}", self.value, self.max_value)),
                        TextFont {
                            font_size: self.config.height - 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });

        bar_entity
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

impl LayoutBuilder for ResourceBarBuilder {
    fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(self, _padding: UiRect) -> Self {
        self
    }
}

/// Convenience function to create a health bar
pub fn health_bar() -> ResourceBarBuilder {
    ResourceBarBuilder::new().style(ResourceBarStyle::Health)
}

/// Convenience function to create a mana bar
pub fn mana_bar() -> ResourceBarBuilder {
    ResourceBarBuilder::new().style(ResourceBarStyle::Mana)
}

/// Convenience function to create a stamina bar
pub fn stamina_bar() -> ResourceBarBuilder {
    ResourceBarBuilder::new().style(ResourceBarStyle::Stamina)
}

/// Convenience function to create an experience bar
pub fn experience_bar() -> ResourceBarBuilder {
    ResourceBarBuilder::new().style(ResourceBarStyle::Experience)
}

//! Resource bar types and components

use bevy::prelude::*;

/// Component marking a resource bar
#[derive(Component, Clone, Debug)]
pub struct ResourceBar {
    /// Current value
    pub value: f32,
    /// Maximum value
    pub max_value: f32,
    /// Visual style
    pub style: ResourceBarStyle,
    /// Whether to animate changes
    pub animated: bool,
}

impl ResourceBar {
    /// Get the current percentage (0.0 to 1.0)
    pub fn percentage(&self) -> f32 {
        if self.max_value <= 0.0 {
            0.0
        } else {
            (self.value / self.max_value).clamp(0.0, 1.0)
        }
    }
}

/// Component for the fill portion of the bar
#[derive(Component, Clone, Debug)]
pub struct ResourceBarFill {
    /// The bar this fill belongs to
    pub bar: Entity,
    /// Target percentage for animation
    pub target_percentage: f32,
    /// Current display percentage
    pub display_percentage: f32,
}

/// Component for animated "damage" indicator
#[derive(Component, Clone, Debug)]
pub struct ResourceBarDamageIndicator {
    /// The bar this indicator belongs to
    pub bar: Entity,
    /// Current display percentage
    pub display_percentage: f32,
    /// Delay before shrinking
    pub delay_timer: f32,
}

/// Visual style presets for resource bars
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceBarStyle {
    /// Health bar (red)
    #[default]
    Health,
    /// Mana bar (blue)
    Mana,
    /// Stamina bar (yellow/green)
    Stamina,
    /// Experience bar (purple)
    Experience,
    /// Shield bar (cyan)
    Shield,
    /// Custom style (use colors directly)
    Custom,
}

impl ResourceBarStyle {
    /// Get the fill color for this style
    pub fn fill_color(&self) -> Color {
        match self {
            Self::Health => Color::srgb(0.8, 0.2, 0.2),
            Self::Mana => Color::srgb(0.2, 0.4, 0.9),
            Self::Stamina => Color::srgb(0.8, 0.8, 0.2),
            Self::Experience => Color::srgb(0.6, 0.2, 0.8),
            Self::Shield => Color::srgb(0.2, 0.8, 0.9),
            Self::Custom => Color::WHITE,
        }
    }

    /// Get the background color for this style
    pub fn background_color(&self) -> Color {
        match self {
            Self::Health => Color::srgb(0.3, 0.1, 0.1),
            Self::Mana => Color::srgb(0.1, 0.15, 0.3),
            Self::Stamina => Color::srgb(0.3, 0.3, 0.1),
            Self::Experience => Color::srgb(0.2, 0.1, 0.25),
            Self::Shield => Color::srgb(0.1, 0.25, 0.3),
            Self::Custom => Color::srgb(0.1, 0.1, 0.1),
        }
    }

    /// Get the damage indicator color
    pub fn damage_color(&self) -> Color {
        match self {
            Self::Health => Color::srgb(1.0, 0.9, 0.9),
            Self::Mana => Color::srgb(0.9, 0.9, 1.0),
            Self::Stamina => Color::srgb(1.0, 1.0, 0.9),
            Self::Experience => Color::srgb(0.95, 0.9, 1.0),
            Self::Shield => Color::srgb(0.9, 1.0, 1.0),
            Self::Custom => Color::WHITE,
        }
    }
}

/// Configuration for resource bar appearance
#[derive(Clone, Debug)]
pub struct ResourceBarConfig {
    /// Fill color
    pub fill_color: Color,
    /// Background color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Damage indicator color
    pub damage_color: Color,
    /// Bar height
    pub height: f32,
    /// Border width
    pub border_width: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Show damage indicator
    pub show_damage_indicator: bool,
    /// Animation speed
    pub animation_speed: f32,
}

impl Default for ResourceBarConfig {
    fn default() -> Self {
        Self {
            fill_color: ResourceBarStyle::Health.fill_color(),
            background_color: ResourceBarStyle::Health.background_color(),
            border_color: Color::srgb(0.3, 0.3, 0.3),
            damage_color: ResourceBarStyle::Health.damage_color(),
            height: 24.0,
            border_width: 2.0,
            corner_radius: 4.0,
            show_damage_indicator: true,
            animation_speed: 3.0,
        }
    }
}

/// Message emitted when a resource bar value changes
#[derive(Message, Clone, Debug)]
pub struct ResourceBarChanged {
    /// The bar entity
    pub bar: Entity,
    /// Previous value
    pub old_value: f32,
    /// New value
    pub new_value: f32,
    /// Maximum value
    pub max_value: f32,
}

/// Global resource bar settings
#[derive(Resource, Clone, Debug)]
pub struct ResourceBarSettings {
    /// Default animation speed
    pub animation_speed: f32,
    /// Damage indicator delay
    pub damage_indicator_delay: f32,
}

impl Default for ResourceBarSettings {
    fn default() -> Self {
        Self {
            animation_speed: 3.0,
            damage_indicator_delay: 0.5,
        }
    }
}

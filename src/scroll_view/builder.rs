//! ScrollView builder for creating scrollable containers with dynamic sizing

use bevy::prelude::*;
use super::types::*;

/// Builder for creating scrollable containers with responsive sizing
pub struct ScrollViewBuilder {
    width: Val,
    height: Val,
    max_width: Val,
    max_height: Val,
    padding: UiRect,
    margin: UiRect,
    gap: Val,
    direction: ScrollDirection,
    config: ScrollConfig,
    background_color: Color,
}

impl ScrollViewBuilder {
    /// Create a new ScrollViewBuilder with sensible defaults
    pub fn new() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Auto,
            max_width: Val::Percent(100.0),
            max_height: Val::Vh(90.0), // 90% viewport height by default
            padding: UiRect::all(Val::Vw(2.0)), // 2% viewport width padding
            margin: UiRect::ZERO,
            gap: Val::Vh(2.0), // 2% viewport height gap
            direction: ScrollDirection::Vertical,
            config: ScrollConfig::default(),
            background_color: Color::NONE,
        }
    }

    /// Set the width of the scroll container
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the scroll container
    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    /// Set the maximum width (useful for responsive design)
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.max_width = max_width;
        self
    }

    /// Set the maximum height (prevents infinite scrolling)
    pub fn max_height(mut self, max_height: Val) -> Self {
        self.max_height = max_height;
        self
    }

    /// Set padding using viewport height percentage
    pub fn padding_vh(mut self, vh: f32) -> Self {
        self.padding = UiRect::all(Val::Vh(vh));
        self
    }

    /// Set padding using viewport width percentage
    pub fn padding_vw(mut self, vw: f32) -> Self {
        self.padding = UiRect::all(Val::Vw(vw));
        self
    }

    /// Set custom padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Set margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    /// Set the gap between child elements (row_gap for vertical, column_gap for horizontal)
    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    /// Set the scroll direction
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Enable/disable auto-scroll to focused elements
    pub fn auto_scroll(mut self, enabled: bool) -> Self {
        self.config.auto_scroll_to_focus = enabled;
        self
    }

    /// Show/hide scroll indicators
    pub fn show_indicators(mut self, show: bool) -> Self {
        self.config.show_indicators = show;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Build the scroll view container and return its entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let overflow = match self.direction {
            ScrollDirection::Vertical => Overflow::scroll_y(),
            ScrollDirection::Horizontal => Overflow::scroll_x(),
            ScrollDirection::Both => Overflow::scroll(),
        };

        let (flex_direction, row_gap, column_gap) = match self.direction {
            ScrollDirection::Vertical => (FlexDirection::Column, self.gap, Val::ZERO),
            ScrollDirection::Horizontal => (FlexDirection::Row, Val::ZERO, self.gap),
            ScrollDirection::Both => (FlexDirection::Column, self.gap, self.gap),
        };

        let show_indicators = self.config.show_indicators;
        let container = parent.spawn((
            Node {
                width: self.width,
                height: self.height,
                max_width: self.max_width,
                max_height: self.max_height,
                padding: self.padding,
                margin: self.margin,
                flex_direction,
                row_gap,
                column_gap,
                align_items: AlignItems::Stretch,
                overflow,
                ..default()
            },
            BackgroundColor(self.background_color),
            ScrollView,
            ScrollState::default(),
            self.config,
        )).id();

        // Add visual scrollbar if indicators are enabled
        if show_indicators && self.direction != ScrollDirection::Horizontal {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                    width: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                ScrollBarTrack,
            )).with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0), // Will be dynamically sized
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    ScrollBarThumb { scroll_container: container },
                    Visibility::Hidden, // Hidden initially
                ));
            });
        }

        container
    }

    /// Build the scroll view and add children using a closure
    pub fn build_with_children<F>(self, parent: &mut ChildSpawnerCommands, children_fn: F) -> Entity
    where
        F: FnOnce(&mut ChildSpawnerCommands),
    {
        let overflow = match self.direction {
            ScrollDirection::Vertical => Overflow::scroll_y(),
            ScrollDirection::Horizontal => Overflow::scroll_x(),
            ScrollDirection::Both => Overflow::scroll(),
        };

        let (flex_direction, row_gap, column_gap) = match self.direction {
            ScrollDirection::Vertical => (FlexDirection::Column, self.gap, Val::ZERO),
            ScrollDirection::Horizontal => (FlexDirection::Row, Val::ZERO, self.gap),
            ScrollDirection::Both => (FlexDirection::Column, self.gap, self.gap),
        };

        let show_indicators = self.config.show_indicators;
        let container = parent
            .spawn((
                Node {
                    width: self.width,
                    height: self.height,
                    max_width: self.max_width,
                    max_height: self.max_height,
                    padding: self.padding,
                    margin: self.margin,
                    flex_direction,
                    row_gap,
                    column_gap,
                    align_items: AlignItems::Stretch,
                    overflow,
                    ..default()
                },
                BackgroundColor(self.background_color),
                ScrollView,
                ScrollState::default(),
                self.config,
            ))
            .with_children(children_fn)
            .id();

        // Add visual scrollbar if indicators are enabled
        if show_indicators && self.direction != ScrollDirection::Horizontal {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                    width: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                ScrollBarTrack,
            )).with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0), // Will be dynamically sized
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    ScrollBarThumb { scroll_container: container },
                    Visibility::Hidden, // Hidden initially
                ));
            });
        }

        container
    }
}

impl Default for ScrollViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create a ScrollViewBuilder
pub fn scroll_view() -> ScrollViewBuilder {
    ScrollViewBuilder::new()
}
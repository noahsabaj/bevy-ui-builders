//! ScrollView builder for creating scrollable containers with dynamic sizing

use bevy::prelude::*;
use super::types::*;

/// Marker component for scrollbar thumbs
#[derive(Component, Debug, Clone, Default)]
pub struct ScrollbarThumb;

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

    /// Set scrollbar visibility mode
    pub fn scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.config.scrollbar_visibility = visibility;
        self
    }

    /// Enable/disable drag-to-scroll
    pub fn enable_drag_scroll(mut self, enabled: bool) -> Self {
        self.config.enable_drag_scroll = enabled;
        self
    }

    /// Enable/disable kinetic scrolling
    pub fn enable_kinetic_scroll(mut self, enabled: bool) -> Self {
        self.config.enable_kinetic_scroll = enabled;
        self
    }

    /// Set scroll sensitivity multiplier
    pub fn scroll_sensitivity(mut self, sensitivity: f32) -> Self {
        self.config.scroll_sensitivity = sensitivity;
        self
    }

    /// Set scrollbar width in pixels
    pub fn scrollbar_width(mut self, width: f32) -> Self {
        self.config.scrollbar_width = width;
        self
    }

    /// Set minimum scrollbar thumb length in pixels
    pub fn min_thumb_length(mut self, length: f32) -> Self {
        self.config.min_thumb_length = length;
        self
    }

    /// Show/hide scroll indicators (deprecated - use scrollbar_visibility instead)
    pub fn show_indicators(mut self, show: bool) -> Self {
        self.config.scrollbar_visibility = if show {
            ScrollbarVisibility::AutoHide { timeout_secs: 2.0 }
        } else {
            ScrollbarVisibility::Never
        };
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

        let show_scrollbar = self.config.scrollbar_visibility != ScrollbarVisibility::Never;
        let enable_drag = self.config.enable_drag_scroll;
        let enable_kinetic = self.config.enable_kinetic_scroll;
        let scrollbar_width = self.config.scrollbar_width;
        let min_thumb_length = self.config.min_thumb_length;

        let mut container_bundle = (
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
            ScrollPosition::default(),
            self.config,
            Interaction::default(), // Required for hover detection in scroll systems
        );

        let container = parent.spawn(container_bundle).id();

        // Add drag-to-scroll capability if enabled
        if enable_drag {
            parent.commands().entity(container).insert(DragScrollTarget);
        }

        // Add kinetic scrolling state if enabled
        if enable_kinetic {
            parent.commands().entity(container).insert(KineticScrollState::default());
        }

        // Add visual scrollbar if not Never visibility
        if show_scrollbar && self.direction != ScrollDirection::Horizontal {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                    width: Val::Px(scrollbar_width),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                ScrollbarState::new(container),
            )).with_children(|scrollbar| {
                scrollbar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0), // Dynamically updated by Bevy
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
                    ScrollbarThumb,
                    Interaction::default(), // Required for drag detection
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

        let show_scrollbar = self.config.scrollbar_visibility != ScrollbarVisibility::Never;
        let enable_drag = self.config.enable_drag_scroll;
        let enable_kinetic = self.config.enable_kinetic_scroll;
        let scrollbar_width = self.config.scrollbar_width;
        let min_thumb_length = self.config.min_thumb_length;

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
                ScrollPosition::default(),
                self.config,
                Interaction::default(), // Required for hover detection in scroll systems
            ))
            .with_children(children_fn)
            .id();

        // Add drag-to-scroll capability if enabled
        if enable_drag {
            parent.commands().entity(container).insert(DragScrollTarget);
        }

        // Add kinetic scrolling state if enabled
        if enable_kinetic {
            parent.commands().entity(container).insert(KineticScrollState::default());
        }

        // Add visual scrollbar if not Never visibility
        if show_scrollbar && self.direction != ScrollDirection::Horizontal {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                    width: Val::Px(scrollbar_width),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                ScrollbarState::new(container),
            )).with_children(|scrollbar| {
                scrollbar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0), // Dynamically updated by Bevy
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
                    ScrollbarThumb,
                    Interaction::default(), // Required for drag detection
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

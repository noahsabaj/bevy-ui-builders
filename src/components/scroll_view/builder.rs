//! ScrollView builder for creating scrollable containers with dynamic sizing

use bevy::prelude::*;
use super::types::*;
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};

/// Marker component for scrollbar thumbs
#[derive(Component, Debug, Clone, Default)]
pub struct ScrollbarThumb;

/// Builder for creating scrollable containers with responsive sizing
pub struct ScrollViewBuilder {
    gap: Val,
    direction: ScrollDirection,
    config: ScrollConfig,
    background_color: Color,
    // Theme-resolved values (set via .themed())
    themed_scrollbar_colors: Option<ScrollbarColors>,
    base: BuilderBase,
}

impl ScrollViewBuilder {
    /// Create a new ScrollViewBuilder with sensible defaults
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Percent(100.0);
        base.node.height = Val::Auto;
        base.node.max_width = Val::Percent(100.0);
        base.node.max_height = Val::Vh(90.0); // 90% viewport height by default
        base.node.padding = UiRect::all(Val::Vw(2.0)); // 2% viewport width padding
        base.node.margin = UiRect::ZERO;

        Self {
            gap: Val::Vh(2.0), // 2% viewport height gap
            direction: ScrollDirection::Vertical,
            config: ScrollConfig::default(),
            background_color: Color::NONE,
            themed_scrollbar_colors: None,
            base,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling for scrollbars.
    /// If not called, sensible defaults will be used.
    ///
    /// # Example
    /// ```ignore
    /// fn setup(mut commands: Commands, theme: Res<UiTheme>) {
    ///     commands.spawn(Node::default()).with_children(|parent| {
    ///         ScrollViewBuilder::new()
    ///             .themed(&theme)
    ///             .build_with_children(parent, |content| {
    ///                 // children here
    ///             });
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_scrollbar_colors = Some(ScrollbarColors::from_theme(theme));
        self
    }

    /// Set the width of the scroll container
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Set the height of the scroll container
    pub fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    /// Set the maximum width (useful for responsive design)
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.base.node.max_width = max_width;
        self
    }

    /// Set the maximum height (prevents infinite scrolling)
    pub fn max_height(mut self, max_height: Val) -> Self {
        self.base.node.max_height = max_height;
        self
    }

    /// Set padding using viewport height percentage
    pub fn padding_vh(mut self, vh: f32) -> Self {
        self.base.node.padding = UiRect::all(Val::Vh(vh));
        self
    }

    /// Set padding using viewport width percentage
    pub fn padding_vw(mut self, vw: f32) -> Self {
        self.base.node.padding = UiRect::all(Val::Vw(vw));
        self
    }

    /// Set custom padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    /// Set margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
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

    /// Build the scroll view container and return its entity (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }

    /// Build the scroll view and add children using a closure
    pub fn build_with_children<F>(mut self, parent: &mut ChildSpawnerCommands, children_fn: F) -> Entity
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
        let _min_thumb_length = self.config.min_thumb_length;

        // Resolve scrollbar colors: themed > default
        let scrollbar_colors = self.themed_scrollbar_colors
            .unwrap_or_else(ScrollbarColors::default_colors);

        // Update base.node with calculated properties
        self.base.node.flex_direction = flex_direction;
        self.base.node.row_gap = row_gap;
        self.base.node.column_gap = column_gap;
        self.base.node.align_items = AlignItems::Stretch;
        self.base.node.overflow = overflow;

        let container = parent
            .spawn((
                self.base.node,
                BackgroundColor(self.background_color),
                ScrollView,
                ScrollPosition::default(),
                self.config,
                Interaction::default(), // Required for hover detection in scroll systems
                Transform::default(), // Required to prevent B0004 warnings
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
                BackgroundColor(scrollbar_colors.track),
                ScrollbarState::new(container),
                Transform::default(), // Required to prevent B0004 warnings
            )).with_children(|scrollbar| {
                scrollbar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0), // Dynamically updated by Bevy
                        ..default()
                    },
                    BackgroundColor(scrollbar_colors.thumb),
                    BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
                    ScrollbarThumb,
                    Interaction::default(), // Required for drag detection
                ));
            });
        }

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(container));
        }

        container
    }
}

impl UiBuilder for ScrollViewBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        self.build_with_children(parent, |_| {})
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

impl LayoutBuilder for ScrollViewBuilder {
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

impl Default for ScrollViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create a ScrollViewBuilder
pub fn scroll_view() -> ScrollViewBuilder {
    ScrollViewBuilder::new()
}

//! TabViewBuilder implementation using type-state pattern

use bevy::prelude::*;
use std::marker::PhantomData;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Type-state marker: No tabs added yet
pub struct NoTabs;

/// Type-state marker: At least one tab has been added
pub struct HasTabs;

/// Builder for creating tab views with compile-time safety
///
/// Uses the type-state pattern to ensure at least one tab is added
/// before building.
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// // This compiles - has tabs
/// TabViewBuilder::new()
///     .themed(&theme)
///     .tab("General", |content| {
///         content.spawn(Text::new("General settings"));
///     })
///     .tab("Advanced", |content| {
///         content.spawn(Text::new("Advanced settings"));
///     })
///     .build(parent);
///
/// // This won't compile - no tabs!
/// // TabViewBuilder::new().build(parent);  // ERROR!
/// ```
pub struct TabViewBuilder<S> {
    tabs: Vec<TabDefinition>,
    style: TabStyle,
    position: TabPosition,
    active_tab: usize,
    // Theme-resolved colors (set via .themed())
    themed_colors: Option<TabColors>,
    base: BuilderBase,
    _state: PhantomData<S>,
}

/// Internal tab definition with content spawner
struct TabDefinition {
    config: TabConfig,
    content_spawner: Box<dyn FnOnce(&mut ChildSpawnerCommands)>,
}

impl TabViewBuilder<NoTabs> {
    /// Create a new tab view builder
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            style: TabStyle::default(),
            position: TabPosition::default(),
            active_tab: 0,
            themed_colors: None,
            base: BuilderBase::new(),
            _state: PhantomData,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(TabColors::from_theme(theme));
        self
    }

    /// Add the first tab (transitions to HasTabs state)
    pub fn tab(
        self,
        label: impl Into<String>,
        content: impl FnOnce(&mut ChildSpawnerCommands) + 'static,
    ) -> TabViewBuilder<HasTabs> {
        let mut tabs = self.tabs;
        tabs.push(TabDefinition {
            config: TabConfig::new(label),
            content_spawner: Box::new(content),
        });

        TabViewBuilder {
            tabs,
            style: self.style,
            position: self.position,
            active_tab: self.active_tab,
            themed_colors: self.themed_colors,
            base: self.base,
            _state: PhantomData,
        }
    }

    /// Add the first tab with full configuration (transitions to HasTabs state)
    pub fn tab_with_config(
        self,
        config: TabConfig,
        content: impl FnOnce(&mut ChildSpawnerCommands) + 'static,
    ) -> TabViewBuilder<HasTabs> {
        let mut tabs = self.tabs;
        tabs.push(TabDefinition {
            config,
            content_spawner: Box::new(content),
        });

        TabViewBuilder {
            tabs,
            style: self.style,
            position: self.position,
            active_tab: self.active_tab,
            themed_colors: self.themed_colors,
            base: self.base,
            _state: PhantomData,
        }
    }

    /// Set the tab style
    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the tab position
    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }
}

impl Default for TabViewBuilder<NoTabs> {
    fn default() -> Self {
        Self::new()
    }
}

impl TabViewBuilder<HasTabs> {
    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(TabColors::from_theme(theme));
        self
    }

    /// Add another tab
    pub fn tab(
        mut self,
        label: impl Into<String>,
        content: impl FnOnce(&mut ChildSpawnerCommands) + 'static,
    ) -> Self {
        self.tabs.push(TabDefinition {
            config: TabConfig::new(label),
            content_spawner: Box::new(content),
        });
        self
    }

    /// Add another tab with full configuration
    pub fn tab_with_config(
        mut self,
        config: TabConfig,
        content: impl FnOnce(&mut ChildSpawnerCommands) + 'static,
    ) -> Self {
        self.tabs.push(TabDefinition {
            config,
            content_spawner: Box::new(content),
        });
        self
    }

    /// Set the tab style
    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the tab position
    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }

    /// Set which tab is initially active
    pub fn active(mut self, index: usize) -> Self {
        self.active_tab = index;
        self
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> TabColors {
        self.themed_colors.clone()
            .unwrap_or_else(TabColors::default_colors)
    }
}

impl UiBuilder for TabViewBuilder<HasTabs> {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Resolve colors (themed > default)
        let colors = self.resolve_colors();

        let tab_count = self.tabs.len();
        let active_tab = self.active_tab.min(tab_count.saturating_sub(1));
        let style = self.style;

        // Determine layout direction based on tab position
        let (container_direction, tabs_direction) = match self.position {
            TabPosition::Top => (FlexDirection::Column, FlexDirection::Row),
            TabPosition::Bottom => (FlexDirection::ColumnReverse, FlexDirection::Row),
            TabPosition::Left => (FlexDirection::Row, FlexDirection::Column),
            TabPosition::Right => (FlexDirection::RowReverse, FlexDirection::Column),
        };

        // Clone the node to avoid partial move issues
        let node = Node {
            flex_direction: container_direction,
            width: Val::Percent(100.0),
            ..default()
        };

        // Pre-spawn tab view to get its entity ID
        let tab_view_entity = parent.spawn_empty().id();

        // Build tab configs before consuming self.tabs
        let tab_configs: Vec<_> = self.tabs.iter().map(|t| t.config.clone()).collect();

        // Spawn the tab view structure
        parent.commands().entity(tab_view_entity).insert((
            node,
            TabView {
                active_tab,
                tab_count,
            },
        )).with_children(|container| {
            // Tab buttons container
            container
                .spawn((
                    Node {
                        flex_direction: tabs_direction,
                        column_gap: Val::Px(4.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(colors.background),
                ))
                .with_children(|tabs_container| {
                    for (index, tab_config) in tab_configs.iter().enumerate() {
                        let is_active = index == active_tab;
                        let (bg_color, text_color) = if is_active {
                            (colors.active_background, colors.active_text)
                        } else if tab_config.disabled {
                            (Color::NONE, colors.disabled_text)
                        } else {
                            (Color::NONE, colors.inactive_text)
                        };

                        tabs_container.spawn((
                            Node {
                                padding: UiRect::new(
                                    Val::Px(16.0),
                                    Val::Px(16.0),
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                ),
                                border: if is_active && style == TabStyle::Line {
                                    UiRect::bottom(Val::Px(2.0))
                                } else {
                                    UiRect::ZERO
                                },
                                ..default()
                            },
                            BackgroundColor(bg_color),
                            BorderColor::all(if is_active { colors.active_background } else { Color::NONE }),
                            BorderRadius::all(Val::Px(
                                if style == TabStyle::Pills { 16.0 } else { 4.0 }
                            )),
                            TabButton {
                                tab_view: tab_view_entity,
                                index,
                            },
                            Interaction::default(),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(&tab_config.label),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_MEDIUM,
                                    ..default()
                                },
                                TextColor(text_color),
                            ));

                            // Badge if present
                            if let Some(count) = tab_config.badge {
                                button.spawn((
                                    Node {
                                        margin: UiRect::left(Val::Px(8.0)),
                                        padding: UiRect::new(
                                            Val::Px(6.0),
                                            Val::Px(6.0),
                                            Val::Px(2.0),
                                            Val::Px(2.0),
                                        ),
                                        ..default()
                                    },
                                    BackgroundColor(colors.badge_background),
                                    BorderRadius::all(Val::Px(10.0)),
                                ))
                                .with_children(|badge| {
                                    badge.spawn((
                                        Text::new(count.to_string()),
                                        TextFont {
                                            font_size: dimensions::FONT_SIZE_SMALL,
                                            ..default()
                                        },
                                        TextColor(colors.badge_text),
                                    ));
                                });
                            }
                        });
                    }
                });

            // Content panels container
            container
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|content_container| {
                    for (index, tab) in self.tabs.drain(..).enumerate() {
                        let is_active = index == active_tab;

                        content_container
                            .spawn((
                                Node {
                                    display: if is_active { Display::Flex } else { Display::None },
                                    width: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                TabContent {
                                    tab_view: tab_view_entity,
                                    index,
                                },
                            ))
                            .with_children(tab.content_spawner);
                    }
                });
        });

        self.base.apply(tab_view_entity, &mut parent.commands());
        tab_view_entity
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

impl LayoutBuilder for TabViewBuilder<HasTabs> {
    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
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
}

/// Convenience function to create a tab view builder
pub fn tabs() -> TabViewBuilder<NoTabs> {
    TabViewBuilder::new()
}

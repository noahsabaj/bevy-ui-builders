//! ProgressBarBuilder implementation

use bevy::prelude::*;
use crate::dimensions;
use crate::theme::{UiTheme, SemanticVariant};
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating progress bars with consistent styling
pub struct ProgressBarBuilder {
    value: f32,
    style: ProgressBarStyle,
    variant: SemanticVariant,
    track_height: Option<Val>,
    track_color: Option<Color>,
    fill_color: Option<Color>,
    show_label: bool,
    custom_label: Option<String>,
    animated: bool,
    // Theme-resolved values (set via .themed())
    themed_track_color: Option<Color>,
    themed_fill_color: Option<Color>,
    themed_label_color: Option<Color>,
    base: BuilderBase,
}

impl ProgressBarBuilder {
    /// Create a new progress bar builder with an initial value (0.0 to 1.0)
    pub fn new(value: f32) -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Percent(100.0);
        base.node.margin = UiRect::all(Val::Px(0.0));

        Self {
            value: value.clamp(0.0, 1.0),
            style: ProgressBarStyle::Default,
            variant: SemanticVariant::Primary,
            track_height: None,
            track_color: None,
            fill_color: None,
            show_label: false,
            custom_label: None,
            animated: false,
            themed_track_color: None,
            themed_fill_color: None,
            themed_label_color: None,
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
    ///         ProgressBarBuilder::new(0.75)
    ///             .themed(&theme)
    ///             .variant(SemanticVariant::Success)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_track_color = Some(self.style.track_color_from_theme(theme));
        self.themed_fill_color = Some(self.style.fill_color_from_theme(theme, self.variant));
        self.themed_label_color = Some(self.style.label_color_from_theme(theme));
        self
    }

    /// Set the semantic variant (controls fill color)
    ///
    /// # Example
    /// ```ignore
    /// ProgressBarBuilder::new(0.3)
    ///     .variant(SemanticVariant::Danger)  // Red for low health
    ///     .build(parent);
    /// ```
    pub fn variant(mut self, variant: SemanticVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the style of the progress bar (controls height)
    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the width of the progress bar
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Override the height of the progress bar track
    pub fn height(mut self, height: Val) -> Self {
        self.track_height = Some(height);
        self
    }

    /// Set the margin around the progress bar
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
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

    /// Build the progress bar (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for ProgressBarBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        let track_height = self.track_height.unwrap_or_else(|| Val::Px(self.style.height()));

        // Color priority: custom override > themed > default
        let track_color = self.track_color
            .or(self.themed_track_color)
            .unwrap_or_else(|| self.style.default_track_color());
        let fill_color = self.fill_color
            .or(self.themed_fill_color)
            .unwrap_or_else(|| self.style.default_fill_color(self.variant));
        let label_color = self.themed_label_color
            .unwrap_or_else(|| self.style.default_label_color());

        // Ensure container properties
        self.base.node.flex_direction = FlexDirection::Column;

        let show_label = self.show_label;
        let custom_label = self.custom_label.clone();
        let value = self.value;

        let entity = parent
            .spawn((
                self.base.node,
                BackgroundColor(Color::NONE),
            ))
            .with_children(|container| {
                // Progress bar track (background)
                container
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: track_height,
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
                                width: Val::Percent(value * 100.0),
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
                if show_label {
                    let label_text = custom_label
                        .unwrap_or_else(|| format!("{}%", (value * 100.0) as i32));

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
                                TextColor(label_color),
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
            .id();

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(entity));
        }

        entity
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

impl LayoutBuilder for ProgressBarBuilder {
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
        self.track_height = Some(height);
        self
    }
}

/// Convenience function to create a progress bar builder
pub fn progress(value: f32) -> ProgressBarBuilder {
    ProgressBarBuilder::new(value)
}
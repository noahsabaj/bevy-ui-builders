//! PanelBuilder implementation

use bevy::prelude::*;
use crate::{dimensions, components::label::{LabelBuilder, LabelSize}};
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Builder for creating panels with consistent styling
pub struct PanelBuilder {
    style: PanelStyle,
    custom_background: Option<Color>,
    title: Option<String>,
    custom_border: Option<UiRect>,
    border_color: Option<Color>,
    // Theme-resolved values (set via .themed())
    themed_background: Option<Color>,
    themed_border_color: Option<Color>,
    base: BuilderBase,
}

impl PanelBuilder {
    /// Create a new panel builder
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        // Set Panel defaults
        base.node.padding = UiRect::all(Val::Px(dimensions::PANEL_PADDING));
        base.node.flex_direction = FlexDirection::Column;
        base.node.justify_content = JustifyContent::Start;
        base.node.align_items = AlignItems::Stretch;
        base.node.display = Display::Flex;

        Self {
            style: PanelStyle::Default,
            custom_background: None,
            title: None,
            custom_border: None,
            border_color: None,
            themed_background: None,
            themed_border_color: None,
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
    ///         PanelBuilder::new()
    ///             .themed(&theme)
    ///             .style(PanelStyle::Card)
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_background = Some(self.style.background_color_from_theme(theme));
        self.themed_border_color = Some(self.style.border_color_from_theme(theme));
        self
    }

    /// Set the panel style
    pub fn style(mut self, style: PanelStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the width
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Set the height
    pub fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    /// Set the minimum width
    pub fn min_width(mut self, min_width: Val) -> Self {
        self.base.node.min_width = min_width;
        self
    }

    /// Set the minimum height
    pub fn min_height(mut self, min_height: Val) -> Self {
        self.base.node.min_height = min_height;
        self
    }

    /// Set the maximum width
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.base.node.max_width = max_width;
        self
    }

    /// Set the maximum height
    pub fn max_height(mut self, max_height: Val) -> Self {
        self.base.node.max_height = max_height;
        self
    }

    /// Set the overflow behavior
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.base.node.overflow = overflow;
        self
    }

    /// Make this panel vertically scrollable
    pub fn scrollable(mut self) -> Self {
        self.base.node.overflow = Overflow::scroll_y();
        self
    }

    /// Make this panel scrollable in both directions
    pub fn scrollable_both(mut self) -> Self {
        self.base.node.overflow = Overflow::scroll();
        self
    }

    /// Use responsive padding based on viewport dimensions
    pub fn responsive_padding(mut self) -> Self {
        self.base.node.padding = UiRect::all(Val::Vw(2.0)); // 2% of viewport width
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    /// Set the margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    /// Set the flex direction
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.base.node.flex_direction = direction;
        self
    }

    /// Set the justify content alignment
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.base.node.justify_content = justify;
        self
    }

    /// Set the align items alignment
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.base.node.align_items = align;
        self
    }

    /// Set the position type (Absolute/Relative)
    pub fn position_type(mut self, position: PositionType) -> Self {
        self.base.node.position_type = position;
        self
    }

    /// Set the display mode (Flex/Grid/None)
    pub fn display(mut self, display: Display) -> Self {
        self.base.node.display = display;
        self
    }

    /// Set a custom background color
    pub fn custom_background(mut self, color: Color) -> Self {
        self.custom_background = Some(color);
        self
    }

    /// Set a title for the panel
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set column gap for flex layout
    pub fn column_gap(mut self, gap: Val) -> Self {
        self.base.node.column_gap = gap;
        self
    }

    /// Set row gap for flex layout
    pub fn row_gap(mut self, gap: Val) -> Self {
        self.base.node.row_gap = gap;
        self
    }

    /// Set flex basis
    pub fn flex_basis(mut self, basis: Val) -> Self {
        self.base.node.flex_basis = basis;
        self
    }

    /// Set flex grow
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.base.node.flex_grow = grow;
        self
    }

    /// Set background color (alias for custom_background)
    pub fn background_color(mut self, color: Color) -> Self {
        self.custom_background = Some(color);
        self
    }

    /// Set custom border
    pub fn border(mut self, border: UiRect) -> Self {
        self.custom_border = Some(border);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Build the panel entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }

    /// Build the panel and spawn children into it
    pub fn build_with_children<F>(
        self,
        parent: &mut ChildSpawnerCommands,
        children: F,
    ) -> Entity
    where
        F: FnOnce(&mut ChildSpawnerCommands),
    {
        let entity = self.build(parent);
        parent.commands().entity(entity).with_children(children);
        entity
    }
}

impl UiBuilder for PanelBuilder {
    fn build(mut self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Color priority: custom override > themed > default
        let background_color = self.custom_background
            .or(self.themed_background)
            .unwrap_or_else(|| self.style.default_background_color());

        let border = self.custom_border
            .unwrap_or_else(|| UiRect::all(self.style.border_width()));

        // Apply border to node
        self.base.node.border = border;

        let border_color = self.border_color
            .or(self.themed_border_color)
            .unwrap_or_else(|| self.style.default_border_color());

        let title = self.title.clone();

        let mut panel_entity = parent.spawn((
            self.base.node,
            BackgroundColor(background_color),
            BorderColor::all(border_color),
            Panel { style: self.style },
        ));

        // Add title if provided
        if let Some(title_text) = title {
            panel_entity.with_children(|parent| {
                LabelBuilder::new(title_text)
                    .size(LabelSize::Title)
                    .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                    .build(parent);
            });
        }

        let entity = panel_entity.id();

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

impl LayoutBuilder for PanelBuilder {
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

/// Convenience function to create a panel builder
pub fn panel() -> PanelBuilder {
    PanelBuilder::new()
}
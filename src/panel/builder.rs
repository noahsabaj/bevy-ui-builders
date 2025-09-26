//! PanelBuilder implementation

use bevy::prelude::*;
use crate::{dimensions, label::{LabelBuilder, LabelStyle}};
use super::types::*;

/// Builder for creating panels with consistent styling
pub struct PanelBuilder {
    style: PanelStyle,
    width: Val,
    height: Val,
    min_width: Val,
    min_height: Val,
    max_width: Val,
    max_height: Val,
    padding: UiRect,
    margin: UiRect,
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    position_type: PositionType,
    display: Display,
    overflow: Overflow,
    custom_background: Option<Color>,
    title: Option<String>,
    column_gap: Val,
    row_gap: Val,
    flex_basis: Val,
    flex_grow: f32,
    custom_border: Option<UiRect>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {
            style: PanelStyle::Default,
            width: Val::Auto,
            height: Val::Auto,
            min_width: Val::Auto,
            min_height: Val::Auto,
            max_width: Val::Auto,
            max_height: Val::Auto,
            padding: UiRect::all(Val::Px(dimensions::PANEL_PADDING)),
            margin: UiRect::all(Val::Px(0.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            position_type: PositionType::Relative,
            display: Display::Flex,
            overflow: Overflow::visible(),
            custom_background: None,
            title: None,
            column_gap: Val::Px(0.0),
            row_gap: Val::Px(0.0),
            flex_basis: Val::Auto,
            flex_grow: 0.0,
            custom_border: None,
        }
    }

    pub fn style(mut self, style: PanelStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn min_width(mut self, min_width: Val) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn min_height(mut self, min_height: Val) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn max_width(mut self, max_width: Val) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: Val) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Make this panel vertically scrollable
    pub fn scrollable(mut self) -> Self {
        self.overflow = Overflow::scroll_y();
        self
    }

    /// Make this panel scrollable in both directions
    pub fn scrollable_both(mut self) -> Self {
        self.overflow = Overflow::scroll();
        self
    }

    /// Use responsive padding based on viewport dimensions
    pub fn responsive_padding(mut self) -> Self {
        self.padding = UiRect::all(Val::Vw(2.0)); // 2% of viewport width
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = direction;
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    pub fn position_type(mut self, position: PositionType) -> Self {
        self.position_type = position;
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.display = display;
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
        self.column_gap = gap;
        self
    }

    /// Set row gap for flex layout
    pub fn row_gap(mut self, gap: Val) -> Self {
        self.row_gap = gap;
        self
    }

    /// Set flex basis
    pub fn flex_basis(mut self, basis: Val) -> Self {
        self.flex_basis = basis;
        self
    }

    /// Set flex grow
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.flex_grow = grow;
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

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let background_color = self
            .custom_background
            .unwrap_or_else(|| self.style.background_color());

        let border = self
            .custom_border
            .unwrap_or_else(|| UiRect::all(self.style.border_width()));

        let mut panel_entity = parent.spawn((
            Node {
                width: self.width,
                height: self.height,
                min_width: self.min_width,
                min_height: self.min_height,
                max_width: self.max_width,
                max_height: self.max_height,
                padding: self.padding,
                margin: self.margin,
                border,
                flex_direction: self.flex_direction,
                justify_content: self.justify_content,
                align_items: self.align_items,
                position_type: self.position_type,
                display: self.display,
                overflow: self.overflow,
                column_gap: self.column_gap,
                row_gap: self.row_gap,
                flex_basis: self.flex_basis,
                flex_grow: self.flex_grow,
                ..default()
            },
            BackgroundColor(background_color),
            BorderColor(self.style.border_color()),
            Panel { style: self.style },
        ));

        // Add title if provided
        if let Some(title) = self.title {
            panel_entity.with_children(|parent| {
                LabelBuilder::new(title)
                    .style(LabelStyle::Title)
                    .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                    .build(parent);
            });
        }

        panel_entity.id()
    }

    pub fn build_with_children<F>(
        self,
        parent: &mut ChildSpawnerCommands,
        children: F,
    ) -> Entity
    where
        F: FnOnce(&mut ChildSpawnerCommands),
    {
        let background_color = self
            .custom_background
            .unwrap_or_else(|| self.style.background_color());

        let border = self
            .custom_border
            .unwrap_or_else(|| UiRect::all(self.style.border_width()));
        let title = self.title.clone(); // Clone title for use in closure

        parent
            .spawn((
                Node {
                    width: self.width,
                    height: self.height,
                    min_width: self.min_width,
                    min_height: self.min_height,
                    max_width: self.max_width,
                    max_height: self.max_height,
                    padding: self.padding,
                    margin: self.margin,
                    border,
                    flex_direction: self.flex_direction,
                    justify_content: self.justify_content,
                    align_items: self.align_items,
                    position_type: self.position_type,
                    display: self.display,
                    overflow: self.overflow,
                    column_gap: self.column_gap,
                    row_gap: self.row_gap,
                    flex_basis: self.flex_basis,
                    flex_grow: self.flex_grow,
                    ..default()
                },
                BackgroundColor(background_color),
                BorderColor(self.style.border_color()),
                Panel { style: self.style },
            ))
            .with_children(|parent| {
                // Add title first if provided
                if let Some(title_text) = title {
                    LabelBuilder::new(title_text)
                        .style(LabelStyle::Title)
                        .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                        .build(parent);
                }

                // Then add user-provided children
                children(parent);
            })
            .id()
    }
}

/// Convenience function to create a panel builder
pub fn panel() -> PanelBuilder {
    PanelBuilder::new()
}
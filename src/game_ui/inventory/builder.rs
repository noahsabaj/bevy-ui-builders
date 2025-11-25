//! InventoryGridBuilder implementation

use bevy::prelude::*;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Default colors for inventory grid (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BACKGROUND_SECONDARY: Color = Color::srgb(0.08, 0.08, 0.1);
}

/// Builder for creating inventory grids
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::game_ui::inventory::*;
///
/// // Basic 6x4 inventory grid
/// InventoryGridBuilder::new(6, 4)
///     .build(parent);
///
/// // Customized inventory
/// InventoryGridBuilder::new(8, 6)
///     .slot_size(Val::Px(48.0))
///     .slot_spacing(Val::Px(4.0))
///     .drag_drop(true)
///     .on_slot_click(|event| { /* handle click */ })
///     .build(parent);
/// ```
pub struct InventoryGridBuilder {
    columns: usize,
    rows: usize,
    slot_size: Val,
    slot_spacing: Val,
    drag_drop: bool,
    slot_style: SlotStyle,
    base: BuilderBase,
}

impl InventoryGridBuilder {
    /// Create a new inventory grid with specified dimensions
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
            slot_size: Val::Px(64.0),
            slot_spacing: Val::Px(4.0),
            drag_drop: true,
            slot_style: SlotStyle::default(),
            base: BuilderBase::new(),
        }
    }

    /// Set the slot size
    pub fn slot_size(mut self, size: Val) -> Self {
        self.slot_size = size;
        self
    }

    /// Set the spacing between slots
    pub fn slot_spacing(mut self, spacing: Val) -> Self {
        self.slot_spacing = spacing;
        self
    }

    /// Enable or disable drag and drop
    pub fn drag_drop(mut self, enabled: bool) -> Self {
        self.drag_drop = enabled;
        self
    }

    /// Set custom slot style
    pub fn slot_style(mut self, style: SlotStyle) -> Self {
        self.slot_style = style;
        self
    }
}

impl UiBuilder for InventoryGridBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let slot_count = self.columns * self.rows;

        // Pre-spawn grid entity
        let grid_entity = parent.spawn_empty().id();

        parent.commands().entity(grid_entity).insert((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::px(self.columns as u16,
                    match self.slot_size {
                        Val::Px(px) => px,
                        _ => 64.0,
                    }
                ),
                grid_template_rows: RepeatedGridTrack::px(self.rows as u16,
                    match self.slot_size {
                        Val::Px(px) => px,
                        _ => 64.0,
                    }
                ),
                row_gap: self.slot_spacing,
                column_gap: self.slot_spacing,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(defaults::BACKGROUND_SECONDARY),
            BorderRadius::all(Val::Px(8.0)),
            InventoryGrid {
                columns: self.columns,
                rows: self.rows,
                slot_count,
            },
        )).with_children(|grid| {
            // Spawn slots
            for index in 0..slot_count {
                let row = index / self.columns;
                let column = index % self.columns;

                grid.spawn((
                    Node {
                        width: self.slot_size,
                        height: self.slot_size,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(self.slot_style.border_width)),
                        ..default()
                    },
                    BackgroundColor(self.slot_style.empty_color),
                    BorderColor::all(self.slot_style.border_color),
                    BorderRadius::all(Val::Px(4.0)),
                    InventorySlot {
                        grid: grid_entity,
                        index,
                        row,
                        column,
                        item: None,
                    },
                    Interaction::default(),
                ));
            }
        });

        grid_entity
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

impl LayoutBuilder for InventoryGridBuilder {
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

/// Convenience function to create an inventory grid builder
pub fn inventory_grid(columns: usize, rows: usize) -> InventoryGridBuilder {
    InventoryGridBuilder::new(columns, rows)
}

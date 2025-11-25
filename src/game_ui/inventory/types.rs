//! Inventory types and components

use bevy::prelude::*;

/// Component marking an inventory grid
#[derive(Component, Clone, Debug)]
pub struct InventoryGrid {
    /// Number of columns
    pub columns: usize,
    /// Number of rows
    pub rows: usize,
    /// Total slot count
    pub slot_count: usize,
}

/// Component marking an inventory slot
#[derive(Component, Clone, Debug)]
pub struct InventorySlot {
    /// The inventory grid this slot belongs to
    pub grid: Entity,
    /// Slot index (0-based)
    pub index: usize,
    /// Row position
    pub row: usize,
    /// Column position
    pub column: usize,
    /// Current item in slot (if any)
    pub item: Option<Entity>,
}

/// Component marking an item in the inventory
#[derive(Component, Clone, Debug)]
pub struct InventoryItem {
    /// The slot this item is in
    pub slot: Entity,
    /// Item data identifier (for your game logic)
    pub item_id: String,
    /// Stack size (for stackable items)
    pub stack_size: u32,
    /// Maximum stack size
    pub max_stack: u32,
}

/// Configuration for inventory slot appearance
#[derive(Clone, Debug)]
pub struct SlotStyle {
    /// Background color for empty slots
    pub empty_color: Color,
    /// Background color when hovered
    pub hover_color: Color,
    /// Background color when selected
    pub selected_color: Color,
    /// Border color
    pub border_color: Color,
    /// Border width
    pub border_width: f32,
}

impl Default for SlotStyle {
    fn default() -> Self {
        Self {
            empty_color: Color::srgb(0.12, 0.12, 0.14),    // BACKGROUND_TERTIARY
            hover_color: Color::srgba(1.0, 1.0, 1.0, 0.05), // GHOST_HOVER
            selected_color: Color::srgb(0.25, 0.46, 0.86), // PRIMARY
            border_color: Color::srgb(0.3, 0.3, 0.3),      // BORDER_DEFAULT
            border_width: 1.0,
        }
    }
}

/// Message emitted when a slot is clicked
#[derive(Message, Clone, Debug)]
pub struct SlotClickEvent {
    /// The inventory grid entity
    pub grid: Entity,
    /// The clicked slot entity
    pub slot: Entity,
    /// The slot index
    pub index: usize,
    /// Mouse button used
    pub button: MouseButton,
}

/// Message emitted when an item is dragged
#[derive(Message, Clone, Debug)]
pub struct ItemDragStartEvent {
    /// The inventory grid entity
    pub grid: Entity,
    /// The source slot
    pub from_slot: Entity,
    /// The item being dragged
    pub item: Entity,
}

/// Message emitted when an item is dropped
#[derive(Message, Clone, Debug)]
pub struct ItemDropEvent {
    /// The inventory grid entity
    pub grid: Entity,
    /// The source slot
    pub from_slot: Entity,
    /// The target slot
    pub to_slot: Entity,
    /// The item being dropped
    pub item: Entity,
}

/// Resource tracking drag state
#[derive(Resource, Default)]
pub struct InventoryDragState {
    /// Currently dragged item
    pub dragging: Option<DragInfo>,
}

/// Information about the current drag operation
#[derive(Clone, Debug)]
pub struct DragInfo {
    /// The item being dragged
    pub item: Entity,
    /// The source slot
    pub from_slot: Entity,
    /// The grid the item came from
    pub grid: Entity,
}

/// Global inventory settings
#[derive(Resource, Clone, Debug)]
pub struct InventorySettings {
    /// Enable drag and drop by default
    pub drag_drop_enabled: bool,
    /// Enable right-click context menu
    pub context_menu_enabled: bool,
    /// Enable tooltips on hover
    pub tooltips_enabled: bool,
}

impl Default for InventorySettings {
    fn default() -> Self {
        Self {
            drag_drop_enabled: true,
            context_menu_enabled: true,
            tooltips_enabled: true,
        }
    }
}
